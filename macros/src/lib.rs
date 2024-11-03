use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, Data, DeriveInput, Path, Type };

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
    quote! { #name: #ty }
  });
  let field_setters = fields.iter().map(|f| {
    let name = f.ident.as_ref().unwrap();
    let name_str = name.to_string();
    let ty = &f.ty;

    if let Type::Path(type_path) = ty {
      if let Some(ident) = type_path.path.segments.last() {
        return match ident.ident.to_string().as_str() {
          "String" => quote! { let _ = obj.set(#name_str, &self.#name); },
          "bool" => quote! { let _ = obj.set(#name_str, self.#name); },
          "Option" =>
            quote! { let _ = obj.set(#name_str, match &self.#name { Some(x) => Some(x.to_object(env)), _ => None }); },
          "Node" => quote! { let _ = obj.set(#name_str, self.#name.to_object(env)); },
          "BodyType" => quote! { let _ = obj.set(#name_str, self.#name.to_object()); },
          "Nodes" =>
            quote! { let _ = obj.set(#name_str, self.#name.iter().map(|x| x.to_object(env)).collect::<Vec<JsObject>>()); },
          _ => {
            println!("Ident: {:?}", ident);
            quote! {}
          }
        };
      }
    }

    return quote! {};
  });

  let expanded =
    quote! {
        impl #struct_name {
            pub fn new(#(#func_args),*) -> Box<Self> {
                Box::new(Self {
                    #(#field_inits),*,
                    leading_comments: vec![],
                    trailing_comments: vec![],
                })
            }
        }

        impl NodeTrait for #struct_name {
            fn add_leading_comments(&mut self, comments: crate::parser::node::Node) {
                self.leading_comments.push(comments);
            }

            fn add_trailing_comments(&mut self, comments: crate::parser::node::Node) {
                self.trailing_comments.push(comments);
            }

            fn get_type(&self) -> NodeType {
               #node_type
            }
            
            fn as_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }

            fn to_object(&self, env: Env) -> JsObject {
                let mut obj = env.create_object().unwrap();
                let _ = obj.set("type", #node_type.to_string());
                
                #(#field_setters)*

                obj
            }
        }
    };

  TokenStream::from(expanded)
}
