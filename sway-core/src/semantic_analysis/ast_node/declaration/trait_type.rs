use sway_error::handler::{ErrorEmitted, Handler};

use crate::{
    language::{
        parsed,
        ty::{self, TyTraitType},
    },
    semantic_analysis::{type_check_context::EnforceTypeArguments, TypeCheckContext},
    type_system::*,
    Engines,
};

impl ty::TyTraitType {
    pub(crate) fn type_check(
        handler: &Handler,
        mut ctx: TypeCheckContext,
        trait_type: parsed::TraitTypeDeclaration,
    ) -> Result<Self, ErrorEmitted> {
        let parsed::TraitTypeDeclaration {
            name,
            attributes,
            ty_opt,
            span,
        } = trait_type;

        let engines = ctx.engines();
        let type_engine = engines.te();

        let ty = if let Some(mut ty) = ty_opt {
            ty.type_id = ctx
                .resolve_type(
                    handler,
                    ty.type_id,
                    &ty.span,
                    EnforceTypeArguments::No,
                    None,
                )
                .unwrap_or_else(|err| type_engine.insert(engines, TypeInfo::ErrorRecovery(err)));
            Some(ty)
        } else {
            None
        };

        let trait_type = ty::TyTraitType {
            name,
            attributes,
            ty,
            span,
        };

        Ok(trait_type)
    }

    /// Used to create a stubbed out constant when the constant fails to
    /// compile, preventing cascading namespace errors.
    pub(crate) fn error(_engines: &Engines, decl: parsed::TraitTypeDeclaration) -> TyTraitType {
        let parsed::TraitTypeDeclaration {
            name,
            attributes,
            ty_opt,
            span,
        } = decl;
        TyTraitType {
            name,
            attributes,
            ty: ty_opt,
            span,
        }
    }
}
