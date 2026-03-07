use syn::{Attribute, Ident, ItemFn, ItemStruct, Visibility};

pub trait Documentable {
    fn vis(&self) -> &Visibility;
    fn attrs(&self) -> &[Attribute];
    fn ident(&self) -> &Ident;
}

impl Documentable for ItemFn {
    fn vis(&self) -> &Visibility {
        &self.vis
    }

    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }

    fn ident(&self) -> &Ident {
        &self.sig.ident
    }
}

impl Documentable for ItemStruct {
    fn vis(&self) -> &Visibility {
        &self.vis
    }

    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }

    fn ident(&self) -> &Ident {
        &self.ident
    }
}
