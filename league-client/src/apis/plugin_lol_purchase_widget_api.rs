/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct PluginLolPurchaseWidgetApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolPurchaseWidgetApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolPurchaseWidgetApiClient<C> {
        PluginLolPurchaseWidgetApiClient {
            configuration,
        }
    }
}

pub trait PluginLolPurchaseWidgetApi {
    fn get_lol_purchase_widget_v1_configuration(&self, ) -> Box<dyn Future<Item = crate::models::LolPurchaseWidgetPurchaseWidgetConfig, Error = Error<serde_json::Value>>>;
    fn get_lol_purchase_widget_v1_order_notifications(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolPurchaseWidgetOrderNotificationResource>, Error = Error<serde_json::Value>>>;
    fn get_lol_purchase_widget_v1_purchasable_item(&self, inventory_type: &str, item_id: i64) -> Box<dyn Future<Item = crate::models::LolPurchaseWidgetPurchasableItem, Error = Error<serde_json::Value>>>;
    fn post_lol_purchase_widget_v1_purchasable_items_by_inventory_type(&self, inventory_type: &str, item_ids: Vec<i64>) -> Box<dyn Future<Item = crate::models::LolPurchaseWidgetItemChoices, Error = Error<serde_json::Value>>>;
    fn post_lol_purchase_widget_v1_purchase_items(&self, purchase_request: crate::models::LolPurchaseWidgetPurchaseRequest) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_purchase_widget_v1_validate_items(&self, validation_request: crate::models::LolPurchaseWidgetValidationRequest) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_purchase_widget_v2_purchase_items(&self, purchase_request: crate::models::LolPurchaseWidgetPurchaseRequest) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolPurchaseWidgetApi for PluginLolPurchaseWidgetApiClient<C> {
    fn get_lol_purchase_widget_v1_configuration(&self, ) -> Box<dyn Future<Item = crate::models::LolPurchaseWidgetPurchaseWidgetConfig, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-purchase-widget/v1/configuration".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_purchase_widget_v1_order_notifications(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolPurchaseWidgetOrderNotificationResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-purchase-widget/v1/order-notifications".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_purchase_widget_v1_purchasable_item(&self, inventory_type: &str, item_id: i64) -> Box<dyn Future<Item = crate::models::LolPurchaseWidgetPurchasableItem, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-purchase-widget/v1/purchasable-item".to_string())
        ;
        req = req.with_query_param("inventoryType".to_string(), inventory_type.to_string());
        req = req.with_query_param("itemId".to_string(), item_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn post_lol_purchase_widget_v1_purchasable_items_by_inventory_type(&self, inventory_type: &str, item_ids: Vec<i64>) -> Box<dyn Future<Item = crate::models::LolPurchaseWidgetItemChoices, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-purchase-widget/v1/purchasable-items/{inventoryType}".to_string())
        ;
        req = req.with_path_param("inventoryType".to_string(), inventory_type.to_string());
        req = req.with_body_param(item_ids);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_purchase_widget_v1_purchase_items(&self, purchase_request: crate::models::LolPurchaseWidgetPurchaseRequest) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-purchase-widget/v1/purchaseItems".to_string())
        ;
        req = req.with_body_param(purchase_request);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_purchase_widget_v1_validate_items(&self, validation_request: crate::models::LolPurchaseWidgetValidationRequest) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-purchase-widget/v1/validateItems".to_string())
        ;
        req = req.with_body_param(validation_request);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_purchase_widget_v2_purchase_items(&self, purchase_request: crate::models::LolPurchaseWidgetPurchaseRequest) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-purchase-widget/v2/purchaseItems".to_string())
        ;
        req = req.with_body_param(purchase_request);

        req.execute(self.configuration.borrow())
    }

}