use chrono::prelude::*;
use serde::Serialize;
use thousands::Separable;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
pub struct Receipt {
  purchase_id: String,         // Purchase ID
  items: Vec<Item>,            // Cart items
  gross: i32,                  // Cart total value without any discounts
  discount_value: i32,         // Given discount value
  discount_percentage: u32,    // Discount value
  has_loyalty_card: bool,      //
  loyalty_card_id: String,     // Loyalty card ID
  loyalty_burned_points: i32,  // Burned loyalty points
  loyalty_earned_points: i32,  // Loyalty earned points
  loyalty_level: String,       // Loyalty level (percentage)
  loyalty_balance_before: i32, //
  loyalty_balance_after: i32,  //
  total_gross: i32,            // Total gross after applying discount and burned points
  date: String,                //
}

impl Receipt {
  pub fn new(
    purchase_id: String,
    items: Vec<Item>,
    gross: i32,
    discount_value: i32,
    discount_percentage: u32,
    has_loyalty_card: bool,
    loyalty_card_id: String,
    loyalty_burned_points: i32,
    loyalty_earned_points: i32,
    loyalty_level: String,
    loyalty_balance_before: i32,
    loyalty_balance_after: i32,
    total_gross: i32,
    date: DateTime<Utc>,
  ) -> Self {
    Self {
      purchase_id,
      items,
      gross,
      discount_value,
      discount_percentage,
      has_loyalty_card,
      loyalty_card_id,
      loyalty_burned_points,
      loyalty_earned_points,
      loyalty_level,
      loyalty_balance_before,
      loyalty_balance_after,
      total_gross,
      date: format!(
        "{}-{:0>2}-{:0>2} {:0>2}:{:>2}:{:>2}",
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute(),
        date.second()
      ),
    }
  }

  pub fn to_latex(&self) -> String {
    let template: &'static str = r#"
        \documentclass\{standalone}
        \usepackage\{graphicx}
        \usepackage\{tabto}
        \usepackage[utf8]\{inputenc}
        \usepackage[T1]\{fontenc}
        
        \begin\{document}
          
          \begin\{minipage}[left]\{7cm}
            \centering
              \vspace\{0.1cm}
              \includegraphics[width=50px]\{logo.jpg} \\
              \Huge\{\textbf\{GardenZilla}} \\
              \vspace\{0.2cm}
              \normalsize\{\textmd\{Kert és Otthon}}\\
              \vspace\{1cm}
              \Large\{Nyugtamelléklet}\\
              \vspace\{0.1cm}
              \scriptsize\{(Nem adóügyi bizonylat!)}
              \vspace\{1cm}
            
            \ttfamily
            \small
            
            \begin\{minipage}[left]\{7cm}
              {{for item in items}}
                {item.sku} \tabto\{1.5cm} {item.name}
                \vspace\{0.2cm}
                \newline
                \tabto\{1cm} {item.piece} db \tabto\{5cm} {item.gross_price_total | number} HUF
                \vspace\{0.5cm}
              {{endfor}}
            \end\{minipage}
        
            \vspace\{1.5cm}
              
            \begin\{minipage}[c]\{6cm}
              Összesen:\hfill {gross | number} HUF\\
              Kedvezmény* (egyedi):\hfill -{discount_value | number} HUF \\
              Kedvezmény (Pont):\hfill -{loyalty_burned_points | number} HUF\\
              
              \large\{Fizetendő:}\hfill \textbf\{\underline\{{total_gross | number} HUF}}\\
            \end\{minipage}
            
            \vspace\{0.3cm}
        
            \scriptsize
            
            * A vásárláshoz egyedi kedvezmény\\lett felhasználva, melynek mértéke {discount_percentage}\%
        
            \small
        
            \vspace\{0.7cm}
              
            \begin\{center}
              Törzsvásárlói tájékoztató\\
              \vspace\{0.1cm}
              {{ if has_loyalty_card}}
              \footnotesize\{Kártya azonosító: {loyalty_card_id}}
              \small
              \vspace\{0.2cm}
            
              \begin\{minipage}[l]\{6cm}
                \dotfill
                
                Nyitó egyenleg:\hfill {loyalty_balance_before} pont\\
                Vásárláshoz\\felhasznált pont:\hfill {loyalty_burned_points} pont\\
                Vásárlás után kapott pont:\hfill {loyalty_earned_points} pont\\
                Záró egyenleg:\hfill {loyalty_balance_after} pont
                
                \dotfill
              \end\{minipage}
              {{endif}}
            
            \vspace\{1cm}
            
            Köszönjük,\\hogy nálunk vásárolt!
            
            \scriptsize
            \vspace\{1cm}
            Vásárlás azonosító: {purchase_id}\\
            4522 Nyírtass, Ady út 11.\\
            {date}
            \vspace\{1cm}
            \end\{center}
          \end\{minipage}
          
        \end\{document}
    "#;

    let mut tt = TinyTemplate::new();
    tt.add_template("receipt", template).unwrap();
    tt.add_formatter("number", |i, o| match i.as_u64() {
      Some(n) => {
        o.push_str(&format!("{}", n.separate_with_spaces()));
        Ok(())
      }
      None => Err(tinytemplate::error::Error::GenericError {
        msg: "only number can be formatted".to_string(),
      }),
    });
    tt.render("receipt", &self).unwrap()
  }
}

#[derive(Serialize)]
pub struct Item {
  pub sku: String,
  pub name: String,
  pub piece: u32,
  pub gross_price_total: u32,
}
