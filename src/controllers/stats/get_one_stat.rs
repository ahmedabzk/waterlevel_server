use axum::{extract::{State, Path}, Json};
use serde_json::{json, Value};
use sqlx::{PgPool};
use uuid::Uuid;

use crate::{errors::custom_errors::CustomErrors, models::statsmodel::{ResponseStats}};



pub async fn get_stat_by_id(
    State(db): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>,CustomErrors>{
    let stat = sqlx::query_as::<_,ResponseStats>("SELECT * FROM stats where id=$1")
        .bind(id)
        .fetch_one(&db)
        .await
        .map_err(|err|{
        println!("error getting the stat {:?}", err);
        CustomErrors::InternalServerError
    })?;


    // let stat = ResponseStats{
    //         id:row.id,
    //         chlorine_level:row.chlorine_level,
    //         ph:row.ph,
    //         turbidity:row.turbidity,
    //         water_level:row.water_level,
    //     };

    Ok(Json(json!(stat)))
}