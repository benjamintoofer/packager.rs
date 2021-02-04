extern crate proc_macro;

use std::{collections::HashMap};

use proc_macro::{TokenStream};
use quote::quote;
use syn::{DataEnum, DataStruct, DataUnion, Error, Field, Fields, Meta, MetaNameValue, Expr, ExprCall, token};


mod container;
mod metadata;

use container::Container;

#[macro_use]
macro_rules! container {
  () => {Container{}};
}

fn temp() {
  let cont = container!();
  // cont.get()
}
/*
1. Parse the tokens
2. extract information on
  2a. Struct name (ident)
  2b. Dependencies (constructor with #[inject] fields)
  2c. 
3. #[inject] requires identifier?
4. #[name] requires tag?
BONUS:
  a. Expect all constructor fields for a injectable must have #[inject]
  b. Expect all constructor fields for a injectable must be a trait


Macro will parse all injectables and generate metadata map that the container can reference on how to
build the object. The metadata map will just be  mapping of the tag name and identifier to what the 
the dependencies are. Must have cache for singletons. When the container tries to resolve an object, it 
will refer to the metadata map and get the list of the string dependecies based off of the tag and identifier.
The container will then resolve those depndencies and continue this recrsive cycle until all the dependecies have
been resolved and can be injected into the contructor of the requested struct
*/

#[proc_macro_derive(Injection, attributes(inject, injectable, singleton, name))]
pub fn derive_injection(tokens: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
  let name = ast.ident;

  match ast.data {
      syn::Data::Struct(DataStruct {fields, ..} ) => {
        let temp = extract_fields(fields);
      }
      syn::Data::Enum(DataEnum {enum_token, ..}) => {
        Error::new(enum_token.span, "expected struct item");
      }
      syn::Data::Union(DataUnion {union_token, ..}) => {
        Error::new(union_token.span, "expected struct item");
      }
  };
  let fields = "";
  println!("---------- HERE --------");
  let gen = quote! {
    
    impl crate::core::injector::Factory for #name {
      fn get() -> Box<#name> {
        println!("------------------BOX new = {}", name);
        Box::new(#name{})
      }
    }

    let value = <#name>::get();
  };


  // #name::get();
  // syn::call!()
  // println!("---------- {}", name);
  gen.into()
  // "".parse().unwrap()
}

fn extract_fields(fields: Fields) -> Result<TokenStream, Vec<Error>> {
  let errors: Vec<Error> = fields.iter()
    .filter(|f| !validate_inject_field(f))
    .map(|f|Error::new(f.ident.to_owned().unwrap().span(), format!("Field \"{}\" does not have a inject attribute", f.ident.to_owned().unwrap().to_string())))
    .collect();
  
  if errors.len() > 0 {
    return Err(errors);
  }

  Ok("".parse().unwrap())
}

fn extract_field_type(field: &Field) {
  let field_type  = match field.ty {
    syn::Type::Path(_) => {}
    syn::Type::Slice(_) => {}
    _ => {}
    // These are currently unused
    // syn::Type::Array(_) => {}
    // syn::Type::BareFn(_) => {}
    // syn::Type::Group(_) => {}
    // syn::Type::ImplTrait(_) => {}
    // syn::Type::Infer(_) => {}
    // syn::Type::Macro(_) => {}
    // syn::Type::Never(_) => {}
    // syn::Type::Paren(_) => {}
    // syn::Type::Ptr(_) => {}
    // syn::Type::Reference(_) => {}
    // syn::Type::TraitObject(_) => {}
    // syn::Type::Tuple(_) => {}
    // syn::Type::Verbatim(_) => {}
    // syn::Type::__Nonexhaustive => {}
  };
}

fn vaidate_trait() {

}

fn to_error_token_stream(errors: &Vec<Error>) -> TokenStream {
  let compile_errors = errors.iter().map(Error::to_compile_error);
  let errors_token_stream = quote!(#(#compile_errors)*);
  errors_token_stream.into()
}

fn validate_inject_field(field: &Field) -> bool {
  field.attrs.iter()
    .filter(|attr| attr.path.is_ident("inject"))
    .count().gt(&0usize)
}
// #[proc_macro_derive(Injection, attributes(inject, injectable, singleton, name))]
// pub fn derive_injection(tokens: TokenStream) -> TokenStream {
//   println!("DERIVE INJECTION");
//   let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
//   match ast.data {
//       syn::Data::Struct(temp) => {
//         println!("DATA-STRUCT");
//         // println!("{:#?}",temp.fields)
//         for i in temp.fields.iter() {
//           println!("FIELD IDEN = {:?}", &i.ident);
//           for attr in i.attrs.iter() {
//             // let whatev: syn::DeriveInput = syn::parse().unwrap();
//             if attr.path.is_ident("inject") {
//               // println!("{:#?}", i.colon_token.unwrap().);
//             }
//             println!("FIELD ATTR TOKEN1 = {:?}", attr.path.is_ident("inject"));
//             // println!("FIELD ATTR = {:?}", b.pound_token);
//           }
//           match &i.ty {
//               syn::Type::Array(_) => {
//                 println!("ARRAY!");
//               }
//               syn::Type::BareFn(_) => {
//                 println!("BARE FUNC!");
//               }
//               syn::Type::Group(_) => {
//                 println!("GRUOP!");
//               }
//               syn::Type::ImplTrait(_) => {
//                 println!("IMPL TRAIT!");
//               }
//               syn::Type::Infer(_) => {
//                 println!("INFER!");
//               }
//               syn::Type::Macro(x) => {
//                 println!("MACRO!");
//               }
//               syn::Type::Never(_) => {
//                 println!("NEVER!");
//               }
//               syn::Type::Paren(_) => {
//                 println!("PAREN!");
//               }
//               syn::Type::Path(p) => {
//                 println!("PATH!");
//                 for seg in p.path.segments.iter() {
//                   println!("SEG - {}", seg.ident);
//                   match &seg.arguments {
//                       syn::PathArguments::None => {}
//                       syn::PathArguments::AngleBracketed(angle) => {
//                         for arg in angle.args.iter() {
//                           match arg {
//                               syn::GenericArgument::Lifetime(_) => {}
//                               syn::GenericArgument::Type(ty) => {
//                                 // println!("ARG TYPE - {}", ty);
//                                 match ty {
//                                     syn::Type::Array(_) => {}
//                                     syn::Type::BareFn(_) => {}
//                                     syn::Type::Group(_) => {}
//                                     syn::Type::ImplTrait(_) => {}
//                                     syn::Type::Infer(_) => {}
//                                     syn::Type::Macro(_) => {}
//                                     syn::Type::Never(_) => {}
//                                     syn::Type::Paren(_) => {}
//                                     syn::Type::Path(path) => {
//                                       println!("ARG TYPE - {}", path.path.segments.last().unwrap().ident);
//                                     }
//                                     syn::Type::Ptr(ptr) => {
                                      
//                                     }
//                                     syn::Type::Reference(_) => {}
//                                     syn::Type::Slice(_) => {}
//                                     syn::Type::TraitObject(to) => {
//                                       println!("ARG TYPE; TraitObject");
//                                       for foo in to.bounds.iter() {
//                                         match &foo {
//                                             syn::TypeParamBound::Trait(tra) => {
//                                               println!("ARG TYPE; TRAI - {:?}", &tra.path.get_ident());
//                                             }
//                                             syn::TypeParamBound::Lifetime(_) => {}
//                                         }
//                                       }
                                      
//                                     }
//                                     syn::Type::Tuple(_) => {}
//                                     syn::Type::Verbatim(_) => {}
//                                     syn::Type::__Nonexhaustive => {}
//                                 }
//                               }
//                               syn::GenericArgument::Binding(_) => {}
//                               syn::GenericArgument::Constraint(_) => {}
//                               syn::GenericArgument::Const(_) => {}
//                           }
//                         }
//                       }
//                       syn::PathArguments::Parenthesized(_) => {}
//                   };
                  
//                 }
//                 // println!("path ===== {:?}\n\n", &p.path.segments.)
//               }
//               syn::Type::Ptr(_) => {
//                 println!("PTR!");
//               }
//               syn::Type::Reference(_) => {
//                 println!("REFERNCE!");
//               }
//               syn::Type::Slice(_) => {
//                 println!("SLICE!");
//               }
//               syn::Type::TraitObject(_) => {
//                 println!("TRAIT OBJ!");
//               }
//               syn::Type::Tuple(_) => {
//                 println!("TUPLE!");
//               }
//               syn::Type::Verbatim(_) => {
//                 println!("Verbatim!");
//               }
//               syn::Type::__Nonexhaustive => {
//                 println!("__Nonexhaustive!");
//               }
//           }
//         }
//         // match temp.fields {
//         //     syn::Fields::Named(x) => {
//         //       x.
//         //       // println!("{:}",x == syn::Fields::Named(syn::data::FieldsNamed("")))
//         //       // x.brace_token.
//         //       // println!("{:?}",x.named.);
//         //       x;
//         //     }
//         //     syn::Fields::Unnamed(_) => {}
//         //     syn::Fields::Unit => {}
//         // }
//       }
//       syn::Data::Enum(_) => {
//         println!("DATA-ENUM");
//       }
//       syn::Data::Union(_) => {
//         println!("DATA-UNION");
//       }
//   }
//   "".parse().unwrap()
// }



#[proc_macro_derive(Singleton)]
pub fn derive_singleton(tokens: TokenStream) -> TokenStream {
  println!("tokens: \"{}\"", tokens.to_string());
  
  let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

  println!("BEFORE LOOP");
  

  "pub struct NewDoris { ben : u32, some : u64, daniel : u8 }".parse().unwrap()
}



// static mut BEN: Ben = 5;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
