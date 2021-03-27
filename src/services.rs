use std::env;

use gzlib::proto::{
  cash::cash_client::CashClient,
  commitment::commitment_client::CommitmentClient,
  customer::customer_client::CustomerClient,
  email::email_client::EmailClient,
  invoice::invoice_client::InvoiceClient,
  latex::latex_client::LatexClient,
  loyalty::loyalty_client::LoyaltyClient,
  pricing::pricing_client::PricingClient,
  procurement::procurement_client::ProcurementClient,
  product::product_client::ProductClient,
  purchase::purchase_client::PurchaseClient,
  sku_image::{sku_image_client::SkuImageClient, sku_image_server::SkuImage},
  sku_image_processer::sku_image_processer_client::SkuImageProcesserClient,
  source::source_client::SourceClient,
  stock::stock_client::StockClient,
  upl::upl_client::UplClient,
  user::user_client::UserClient,
};
use tonic::transport::Channel;

// Helper to load service address from env
fn service_address(service_name: &'static str) -> String {
  let addr = env::var(service_name).expect(&format!(
    "Could not get service address for {}",
    service_name
  ));
  format!("http://{}", addr)
}

#[derive(Debug, Clone)]
pub struct Services {
  pub email: EmailClient<Channel>,
  pub user: UserClient<Channel>,
  pub customer: CustomerClient<Channel>,
  pub upl: UplClient<Channel>,
  pub product: ProductClient<Channel>,
  pub source: SourceClient<Channel>,
  pub procurement: ProcurementClient<Channel>,
  pub cash: CashClient<Channel>,
  pub document: (),
  pub invoice: InvoiceClient<Channel>,
  pub pricing: PricingClient<Channel>,
  pub auth: (),
  pub stock: StockClient<Channel>,
  pub purchase: PurchaseClient<Channel>,
  pub latex: LatexClient<Channel>,
  pub commitment: CommitmentClient<Channel>,
  pub loyalty: LoyaltyClient<Channel>,
  pub sku_image: SkuImageClient<Channel>,
  pub sku_img_processer: SkuImageProcesserClient<Channel>,
}

impl Services {
  pub async fn init() -> Self {
    Self {
      email: EmailClient::connect(service_address("SERVICE_ADDR_EMAIL"))
        .await
        .expect("Could not connect to email service"),
      user: UserClient::connect(service_address("SERVICE_ADDR_USER"))
        .await
        .expect("Could not connect to user service"),
      customer: CustomerClient::connect(service_address("SERVICE_ADDR_CUSTOMER"))
        .await
        .expect("Could not connect to customer service"),
      upl: UplClient::connect(service_address("SERVICE_ADDR_UPL"))
        .await
        .expect("Could not connect to upl service"),
      product: ProductClient::connect(service_address("SERVICE_ADDR_PRODUCT"))
        .await
        .expect("Could not connect to product service"),
      source: SourceClient::connect(service_address("SERVICE_ADDR_SOURCE"))
        .await
        .expect("Could not connect to source service"),
      procurement: ProcurementClient::connect(service_address("SERVICE_ADDR_PROCUREMENT"))
        .await
        .expect("Could not connect to procurement service"),
      cash: CashClient::connect(service_address("SERVICE_ADDR_CASH"))
        .await
        .expect("Could not connect to cash service"),
      document: (),
      invoice: InvoiceClient::connect(service_address("SERVICE_ADDR_INVOICE"))
        .await
        .expect("Could not connect to invoice service"),
      pricing: PricingClient::connect(service_address("SERVICE_ADDR_PRICING"))
        .await
        .expect("Could not connect to pricing service"),
      auth: (),
      stock: StockClient::connect(service_address("SERVICE_ADDR_STOCK"))
        .await
        .expect("Could not connect to stock service"),
      purchase: PurchaseClient::connect(service_address("SERVICE_ADDR_PURCHASE"))
        .await
        .expect("Could not connect to purchase service"),
      latex: LatexClient::connect(service_address("SERVICE_ADDR_LATEX"))
        .await
        .expect("Could not connect to latex service"),
      commitment: CommitmentClient::connect(service_address("SERVICE_ADDR_COMMITMENT"))
        .await
        .expect("Could not connect to commitment service"),
      loyalty: LoyaltyClient::connect(service_address("SERVICE_ADDR_LOYALTY"))
        .await
        .expect("Could not connect to loyalty service"),
      sku_image: SkuImageClient::connect(service_address("SERVICE_ADDR_SKU_IMAGE"))
        .await
        .expect("Could not connect to sku image service"),
      sku_img_processer: SkuImageProcesserClient::connect(service_address(
        "SERVICE_ADDR_SKU_IMG_PROCESSER",
      ))
      .await
      .expect("Could not connect to sku image processer service"),
    }
  }
}
