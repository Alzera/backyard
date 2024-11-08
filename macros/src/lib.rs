use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, Data, DeriveInput, Path };

#[proc_macro_derive(ImplementNodeTrait, attributes(implement_node_trait))]
pub fn implement_node_trait(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let struct_name = input.ident;
  let fields = match input.data {
    Data::Struct(data_struct) => data_struct.fields,
    _ => panic!("ImplementNodeTrait can only be used with structs"),
  };

  let mut node_type = quote! { NodeType::Array };
  for attr in input.attrs.iter() {
    if attr.path().is_ident("implement_node_trait") {
      if let Ok(parsed_path) = attr.parse_args::<Path>() {
        node_type = quote! { #parsed_path };
      }
    }
  }

  let field_filtered: Vec<&syn::Field> = fields
    .iter()
    .filter(|f| {
      if let Some(ident) = &f.ident {
        !["trailing_comments", "leading_comments"].contains(&ident.to_string().as_str())
      } else {
        true
      }
    })
    .collect();

  let field_inits = field_filtered.iter().map(|f| {
    let name = &f.ident;
    quote! { #name }
  });
  let func_args = field_filtered.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;
    quote! { #name: #ty, }
  });
  let field_setters = field_filtered.iter().map(|f| {
    let name = f.ident.as_ref().unwrap();
    let name_str = name.to_string();

    return quote! { let _ = obj.set(#name_str, val.#name); };
  });
  let field_getter = field_filtered.iter().map(|f| {
    let name = f.ident.as_ref().unwrap();
    let name_str = name.to_string();
    let ty = &f.ty;

    quote! { #name: val.get::<&str, #ty>(#name_str).unwrap().unwrap() }
  });

  let expanded =
    quote! {
      impl #struct_name {
        pub fn new(#(#func_args)*) -> Box<Self> {
          Box::new(Self {
            #(#field_inits),*,
            leading_comments: vec![],
            trailing_comments: vec![],
          })
        }
      }

      impl napi::bindgen_prelude::ToNapiValue for #struct_name {
        unsafe fn to_napi_value(
          env: napi::sys::napi_env,
          val: Self
        ) -> napi::Result<napi::sys::napi_value> {
          let unraw_env = napi::Env::from_raw(env);
          let mut obj = unraw_env.create_object()?;
          let _ = obj.set("type", #node_type.to_string());
          
          #(#field_setters)*

          if val.leading_comments.len() > 0 {
            let _ = obj.set("leading_comments", val.leading_comments);
          }
          if val.trailing_comments.len() > 0 {
            let _ = obj.set("trailing_comments", val.trailing_comments);
          }

          napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
      }

      impl crate::parser::node::NodeTrait for #struct_name {
        fn add_leading_comments(&mut self, comments: crate::parser::node::Node) {
          self.leading_comments.push(comments);
        }

        fn add_trailing_comments(&mut self, comments: crate::parser::node::Node) {
          self.trailing_comments.push(comments);
        }

        fn get_type(&self) -> crate::parser::node::NodeType {
          #node_type
        }
          
        fn as_any(self: Box<Self>) -> Box<dyn Any> {
          self
        }
          
        unsafe fn to_napi(&self, env: napi::sys::napi_env) -> napi::Result<napi::sys::napi_value> {
          #struct_name::to_napi_value(env, self.clone())
        }

        fn from_napi(env: napi::sys::napi_env, val: napi::JsObject) -> Box<Self> where Self: Sized {
          Box::new(Self {
            #(#field_getter),*,
            leading_comments: crate::guard!(val.get::<&str, Nodes>("leading_comments").unwrap(), vec![]), 
            trailing_comments: crate::guard!(val.get::<&str, Nodes>("trailing_comments").unwrap(), vec![]),
          })
        }
      }
    };

  TokenStream::from(expanded)
}


#[proc_macro_derive(ImplementGeneratorNodeTrait, attributes(implement_generator_node_trait))]
pub fn implement_generator_node_trait(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let struct_name = input.ident;
  let fields = match input.data {
    Data::Struct(data_struct) => data_struct.fields,
    _ => panic!("ImplementGeneratorNodeTrait can only be used with structs"),
  };

  let mut node_type = quote! { NodeType::Array };
  for attr in input.attrs.iter() {
    if attr.path().is_ident("implement_generator_node_trait") {
      if let Ok(parsed_path) = attr.parse_args::<Path>() {
        node_type = quote! { #parsed_path };
      }
    }
  }

  let field_filtered: Vec<&syn::Field> = fields
    .iter()
    .filter(|f| {
      if let Some(ident) = &f.ident {
        !["trailing_comments", "leading_comments"].contains(&ident.to_string().as_str())
      } else {
        true
      }
    })
    .collect();

  let field_inits = field_filtered.iter().map(|f| {
    let name = &f.ident;
    quote! { #name }
  });
  let func_args = field_filtered.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;
    quote! { #name: #ty, }
  });
  let field_setters = field_filtered.iter().map(|f| {
    let name = f.ident.as_ref().unwrap();
    let name_str = name.to_string();

    return quote! { let _ = obj.set(#name_str, val.#name); };
  });
  let field_getter = field_filtered.iter().map(|f| {
    let name = f.ident.as_ref().unwrap();
    let name_str = name.to_string();
    let ty = &f.ty;

    quote! { #name: val.get::<&str, #ty>(#name_str).unwrap().unwrap() }
  });

  let expanded =
    quote! {
      impl #struct_name {
        pub fn new(#(#func_args)*) -> Box<Self> {
          Box::new(Self {
            #(#field_inits),*,
            leading_comments: vec![],
            trailing_comments: vec![],
          })
        }
      }

      impl napi::bindgen_prelude::ToNapiValue for #struct_name {
        unsafe fn to_napi_value(
          env: napi::sys::napi_env,
          val: Self
        ) -> napi::Result<napi::sys::napi_value> {
          let unraw_env = napi::Env::from_raw(env);
          let mut obj = unraw_env.create_object()?;
          let _ = obj.set("type", #node_type.to_string());
          
          #(#field_setters)*

          if val.leading_comments.len() > 0 {
            let _ = obj.set("leading_comments", val.leading_comments);
          }
          if val.trailing_comments.len() > 0 {
            let _ = obj.set("trailing_comments", val.trailing_comments);
          }

          napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
      }

      impl crate::parser::node::NodeTrait for #struct_name {
        fn add_leading_comments(&mut self, comments: crate::parser::node::Node) {
          self.leading_comments.push(comments);
        }

        fn add_trailing_comments(&mut self, comments: crate::parser::node::Node) {
          self.trailing_comments.push(comments);
        }

        fn get_type(&self) -> crate::parser::node::NodeType {
          #node_type
        }
          
        fn as_any(self: Box<Self>) -> Box<dyn Any> {
          self
        }
          
        unsafe fn to_napi(&self, env: napi::sys::napi_env) -> napi::Result<napi::sys::napi_value> {
          #struct_name::to_napi_value(env, self.clone())
        }

        fn from_napi(env: napi::sys::napi_env, val: napi::JsObject) -> Box<Self> where Self: Sized {
          Box::new(Self {
            #(#field_getter),*,
            leading_comments: crate::guard!(val.get::<&str, Nodes>("leading_comments").unwrap(), vec![]), 
            trailing_comments: crate::guard!(val.get::<&str, Nodes>("trailing_comments").unwrap(), vec![]),
          })
        }
      }
    };

  TokenStream::from(expanded)
}