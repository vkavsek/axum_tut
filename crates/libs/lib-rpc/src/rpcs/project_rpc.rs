// use lib_core::{ctx::Ctx, model::ModelManager};
//
// use crate::Result;
// use crate::{
//     params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList},
//     router::RpcRouter,
//     rpc_router,
// };
//
// pub fn rpc_router() -> RpcRouter {
//     rpc_router!(
//         create_project,
//         list_projects,
//         update_project,
//         delete_project
//     )
// }
//
// pub async fn create_project(
//     ctx: Ctx,
//     mm: ModelManager,
//     params: ParamsForCreate<ProjectForCreate>,
// ) -> Result<Project> {
//     let ParamsForCreate { data } = params;
//
//     let id = ProjectBmc::create(&ctx, &mm, data).await?;
//     let task: Project = ProjectBmc::get(&ctx, &mm, id).await?;
//
//     Ok(task)
// }
//
// pub async fn get_project(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Project> {
//     let ParamsIded { id } = params;
//     let task = ProjectBmc::get(&ctx, &mm, id).await?;
//     Ok(task)
// }
//
// pub async fn list_projects(
//     ctx: Ctx,
//     mm: ModelManager,
//     params: ParamsList<ProjectFilter>,
// ) -> Result<Vec<Project>> {
//     let tasks = ProjectBmc::list(&ctx, &mm, params.filters, params.list_options).await?;
//     Ok(tasks)
// }
//
// pub async fn update_project(
//     ctx: Ctx,
//     mm: ModelManager,
//     params: ParamsForUpdate<ProjectForUpdate>,
// ) -> Result<Project> {
//     let ParamsForUpdate { id, data } = params;
//
//     ProjectBmc::update(&ctx, &mm, id, data).await?;
//     let task = ProjectBmc::get(&ctx, &mm, id).await?;
//
//     Ok(task)
// }
//
// pub async fn delete_project(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Project> {
//     let ParamsIded { id } = params;
//
//     let task = ProjectBmc::get(&ctx, &mm, id).await?;
//     ProjectBmc::delete(&ctx, &mm, id).await?;
//
//     Ok(task)
// }
