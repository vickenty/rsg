use std::mem;

use quote::ToTokens;
use syn::{self, visit::Visit};

use sxd_document::dom::{Document, Element};

pub struct Builder<'d, 'ast> {
    pub doc: Document<'d>,
    pub root: Option<Element<'d>>,
    pub items: Vec<&'ast dyn ToTokens>,
}

const ID_ATTR: &str = "__id__";

impl<'d, 'ast> Builder<'d, 'ast> {
    fn add<F>(&mut self, name: &'static str, item: Option<&'ast dyn ToTokens>, f: F)
    where
        F: FnOnce(&mut Self),
    {
        let el = self.doc.create_element(name);

        if let Some(item) = item {
            let idx = format!("{}", self.items.len());
            self.items.push(item);
            el.set_attribute_value(ID_ATTR, &idx);
        }

        if let Some(parent) = &self.root {
            parent.append_child(el);
        } else {
            self.doc.root().append_child(el);
        }

        let prev_root = mem::replace(&mut self.root, Some(el));
        f(self);
        self.root = prev_root;
    }

    pub fn new(doc: Document<'d>) -> Builder<'d, 'ast> {
        Builder {
            doc,
            root: None,
            items: Vec::new(),
        }
    }

    pub fn get(&self, el: Element<'d>) -> Option<&'ast dyn ToTokens> {
        let idx: usize = el.attribute_value(ID_ATTR).unwrap().parse().unwrap();
        self.items.get(idx).copied()
    }
}

macro_rules! visit {
    ( $lt:lifetime, $( $id:ident: $it:ident $(, $opt:tt )?; )* ) => {
        $(
            fn $id(&mut self, i: &$lt syn::$it) {
                self.add(stringify!($it), visit!(? i $( $opt )*), |v| syn::visit::$id(v, i));
            }
        )*
    };

    (? $i:ident ) => { Some($i) };
    (? $i:ident ?) => { None };
}

impl<'d, 'ast> Visit<'ast> for Builder<'d, 'ast> {
    visit! {
        'ast,
        visit_abi: Abi;
        visit_angle_bracketed_generic_arguments: AngleBracketedGenericArguments;
        visit_arm: Arm;
        visit_attr_style: AttrStyle, ?;
        visit_attribute: Attribute;
        visit_bare_fn_arg: BareFnArg;
        visit_bin_op: BinOp;
        visit_binding: Binding;
        visit_block: Block;
        visit_bound_lifetimes: BoundLifetimes;
        visit_const_param: ConstParam;
        visit_constraint: Constraint;
        visit_data: Data, ?;
        visit_data_enum: DataEnum, ?;
        visit_data_struct: DataStruct, ?;
        visit_data_union: DataUnion, ?;
        visit_derive_input: DeriveInput;
        visit_expr: Expr;
        visit_expr_array: ExprArray;
        visit_expr_assign: ExprAssign;
        visit_expr_assign_op: ExprAssignOp;
        visit_expr_async: ExprAsync;
        visit_expr_await: ExprAwait;
        visit_expr_binary: ExprBinary;
        visit_expr_block: ExprBlock;
        visit_expr_box: ExprBox;
        visit_expr_break: ExprBreak;
        visit_expr_call: ExprCall;
        visit_expr_cast: ExprCast;
        visit_expr_closure: ExprClosure;
        visit_expr_continue: ExprContinue;
        visit_expr_field: ExprField;
        visit_expr_for_loop: ExprForLoop;
        visit_expr_group: ExprGroup;
        visit_expr_if: ExprIf;
        visit_expr_index: ExprIndex;
        visit_expr_let: ExprLet;
        visit_expr_lit: ExprLit;
        visit_expr_loop: ExprLoop;
        visit_expr_macro: ExprMacro;
        visit_expr_match: ExprMatch;
        visit_expr_method_call: ExprMethodCall;
        visit_expr_paren: ExprParen;
        visit_expr_path: ExprPath;
        visit_expr_range: ExprRange;
        visit_expr_reference: ExprReference;
        visit_expr_repeat: ExprRepeat;
        visit_expr_return: ExprReturn;
        visit_expr_struct: ExprStruct;
        visit_expr_try: ExprTry;
        visit_expr_try_block: ExprTryBlock;
        visit_expr_tuple: ExprTuple;
        visit_expr_type: ExprType;
        visit_expr_unary: ExprUnary;
        visit_expr_unsafe: ExprUnsafe;
        visit_expr_while: ExprWhile;
        visit_expr_yield: ExprYield;
        visit_field: Field;
        visit_field_pat: FieldPat;
        visit_field_value: FieldValue;
        visit_fields: Fields;
        visit_fields_named: FieldsNamed;
        visit_fields_unnamed: FieldsUnnamed;
        visit_file: File;
        visit_fn_arg: FnArg;
        visit_foreign_item: ForeignItem;
        visit_foreign_item_fn: ForeignItemFn;
        visit_foreign_item_macro: ForeignItemMacro;
        visit_foreign_item_static: ForeignItemStatic;
        visit_foreign_item_type: ForeignItemType;
        visit_generic_argument: GenericArgument;
        visit_generic_method_argument: GenericMethodArgument;
        visit_generic_param: GenericParam;
        visit_generics: Generics;
        // visit_ident: Ident;
        visit_impl_item: ImplItem;
        visit_impl_item_const: ImplItemConst;
        visit_impl_item_macro: ImplItemMacro;
        visit_impl_item_method: ImplItemMethod;
        visit_impl_item_type: ImplItemType;
        visit_index: Index;
        visit_item: Item;
        visit_item_const: ItemConst;
        visit_item_enum: ItemEnum;
        visit_item_extern_crate: ItemExternCrate;
        visit_item_fn: ItemFn;
        visit_item_foreign_mod: ItemForeignMod;
        visit_item_impl: ItemImpl;
        visit_item_macro: ItemMacro;
        visit_item_macro2: ItemMacro2;
        visit_item_mod: ItemMod;
        visit_item_static: ItemStatic;
        visit_item_struct: ItemStruct;
        visit_item_trait: ItemTrait;
        visit_item_trait_alias: ItemTraitAlias;
        visit_item_type: ItemType;
        visit_item_union: ItemUnion;
        visit_item_use: ItemUse;
        visit_label: Label;
        visit_lifetime: Lifetime;
        visit_lifetime_def: LifetimeDef;
        visit_lit: Lit;
        visit_lit_bool: LitBool;
        visit_lit_byte: LitByte;
        visit_lit_byte_str: LitByteStr;
        visit_lit_char: LitChar;
        visit_lit_float: LitFloat;
        visit_lit_int: LitInt;
        visit_lit_str: LitStr;
        visit_local: Local;
        visit_macro: Macro;
        visit_macro_delimiter: MacroDelimiter, ?;
        visit_member: Member;
        visit_meta: Meta;
        visit_meta_list: MetaList;
        visit_meta_name_value: MetaNameValue;
        visit_method_turbofish: MethodTurbofish;
        visit_nested_meta: NestedMeta;
        visit_parenthesized_generic_arguments: ParenthesizedGenericArguments;
        visit_pat: Pat;
        visit_pat_box: PatBox;
        visit_pat_ident: PatIdent;
        visit_pat_lit: PatLit;
        visit_pat_macro: PatMacro;
        visit_pat_or: PatOr;
        visit_pat_path: PatPath;
        visit_pat_range: PatRange;
        visit_pat_reference: PatReference;
        visit_pat_rest: PatRest;
        visit_pat_slice: PatSlice;
        visit_pat_struct: PatStruct;
        visit_pat_tuple: PatTuple;
        visit_pat_tuple_struct: PatTupleStruct;
        visit_pat_type: PatType;
        visit_pat_wild: PatWild;
        visit_path: Path;
        visit_path_arguments: PathArguments;
        visit_path_segment: PathSegment;
        visit_predicate_eq: PredicateEq;
        visit_predicate_lifetime: PredicateLifetime;
        visit_predicate_type: PredicateType;
        visit_qself: QSelf, ?;
        visit_range_limits: RangeLimits, ?;
        visit_receiver: Receiver;
        visit_return_type: ReturnType;
        visit_signature: Signature;
        visit_stmt: Stmt;
        visit_trait_bound: TraitBound;
        visit_trait_bound_modifier: TraitBoundModifier;
        visit_trait_item: TraitItem;
        visit_trait_item_const: TraitItemConst;
        visit_trait_item_macro: TraitItemMacro;
        visit_trait_item_method: TraitItemMethod;
        visit_trait_item_type: TraitItemType;
        visit_type: Type;
        visit_type_array: TypeArray;
        visit_type_bare_fn: TypeBareFn;
        visit_type_group: TypeGroup;
        visit_type_impl_trait: TypeImplTrait;
        visit_type_infer: TypeInfer;
        visit_type_macro: TypeMacro;
        visit_type_never: TypeNever;
        visit_type_param: TypeParam;
        visit_type_param_bound: TypeParamBound;
        visit_type_paren: TypeParen;
        visit_type_path: TypePath;
        visit_type_ptr: TypePtr;
        visit_type_reference: TypeReference;
        visit_type_slice: TypeSlice;
        visit_type_trait_object: TypeTraitObject;
        visit_type_tuple: TypeTuple;
        visit_un_op: UnOp;
        visit_use_glob: UseGlob;
        visit_use_group: UseGroup;
        visit_use_name: UseName;
        visit_use_path: UsePath;
        visit_use_rename: UseRename;
        visit_use_tree: UseTree;
        visit_variadic: Variadic;
        visit_variant: Variant;
        visit_vis_crate: VisCrate;
        visit_vis_public: VisPublic;
        visit_vis_restricted: VisRestricted;
        visit_visibility: Visibility;
        visit_where_clause: WhereClause;
        visit_where_predicate: WherePredicate;
    }

    fn visit_ident(&mut self, i: &'ast syn::Ident) {
        self.add("Ident", Some(i), |v| {
            v.root.unwrap().set_text(&i.to_string());
        });
    }
}
