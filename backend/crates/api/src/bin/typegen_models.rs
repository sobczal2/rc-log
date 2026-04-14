use std::error::Error;
use std::fs;
use std::path::PathBuf;

use rc_log_application::maneuver::get_by_id::model::ManeuverDto as GetByIdManeuverDto;
use rc_log_application::maneuver::get_by_id::model::TagDto as GetByIdTagDto;
use rc_log_application::maneuver::get_by_id::model::VariationDto as GetByIdVariationDto;
use rc_log_application::maneuver::list::model::ManeuverDto as ListManeuverDto;
use rc_log_application::maneuver::list::model::TagDto as ListTagDto;
use rc_log_application::maneuver::shared::difficulty::DifficultyDto;
use rc_log_application::model::create::model::ModelDto as CreateModelDto;
use rc_log_application::model::get_by_id::model::ModelDto as GetByIdModelDto;
use rc_log_application::model::list::model::ModelDto as ListModelDto;
use rc_log_application::model::shared::TypeDto;
use rc_log_application::model::update::model::ModelDto as UpdateModelDto;
use rc_log_application::model::update_photo::model::ModelDto as UpdateModelPhotoDto;
use rc_log_application::photo::resolve::model::PhotoPathsDto;
use rc_log_application::session::add_performed_variation::model::PerformedVariationDto as AddPerformedVariationDto;
use rc_log_application::session::create::model::SessionDto as CreateSessionDto;
use rc_log_application::session::list::model::PerformedVariationDto as ListPerformedVariationDto;
use rc_log_application::session::list::model::SessionDto as ListSessionDto;
use rc_log_application::session::shared::rating::RatingDto;
use rc_log_application::session::update::model::SessionDto as UpdateSessionDto;
use rc_log_application::shared::pagination::PaginationDto;
use rc_log_application::user::get_by_id::model::UserDto as GetByIdUserDto;
use rc_log_application::user::sign_in::model::UserDto as SignInUserDto;
use rc_log_application::user::sign_up::model::UserDto as SignUpUserDto;
use rc_log_application::user::update::model::UserDto as UpdateUserDto;
use rc_log_application::user::update_photo::model::UserDto as UpdateUserPhotoDto;
use rc_log_application::video::resolve::model::VideoPathsDto;
use specta::ts::{ExportConfiguration, export};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = output_dir();
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir)?;
    }
    fs::create_dir_all(&out_dir)?;

    export_file_with_deps(
        &out_dir,
        "model/shared/type.ts",
        &[export::<TypeDto>(&ExportConfiguration::default())?],
    )?;
    export_file_with_deps(
        &out_dir,
        "shared/pagination.ts",
        &[export::<PaginationDto>(&ExportConfiguration::default())?],
    )?;
    export_file_with_deps(
        &out_dir,
        "maneuver/get-by-id.ts",
        &[
            export::<TypeDto>(&ExportConfiguration::default())?,
            export::<DifficultyDto>(&ExportConfiguration::default())?,
            export::<GetByIdTagDto>(&ExportConfiguration::default())?,
            export::<GetByIdVariationDto>(&ExportConfiguration::default())?,
            export::<GetByIdManeuverDto>(&ExportConfiguration::default())?,
        ],
    )?;
    export_file_with_deps(
        &out_dir,
        "maneuver/list.ts",
        &[
            export::<TypeDto>(&ExportConfiguration::default())?,
            export::<DifficultyDto>(&ExportConfiguration::default())?,
            export::<ListTagDto>(&ExportConfiguration::default())?,
            export::<ListManeuverDto>(&ExportConfiguration::default())?,
        ],
    )?;
    export_file_with_deps(
        &out_dir,
        "model/create.ts",
        &[
            export::<TypeDto>(&ExportConfiguration::default())?,
            export::<CreateModelDto>(&ExportConfiguration::default())?,
        ],
    )?;
    export_file_with_deps(
        &out_dir,
        "model/get-by-id.ts",
        &[
            export::<TypeDto>(&ExportConfiguration::default())?,
            export::<GetByIdModelDto>(&ExportConfiguration::default())?,
        ],
    )?;
    export_file_with_deps(
        &out_dir,
        "model/list.ts",
        &[
            export::<TypeDto>(&ExportConfiguration::default())?,
            export::<ListModelDto>(&ExportConfiguration::default())?,
        ],
    )?;
    export_file_with_deps(
        &out_dir,
        "model/update.ts",
        &[
            export::<TypeDto>(&ExportConfiguration::default())?,
            export::<UpdateModelDto>(&ExportConfiguration::default())?,
        ],
    )?;
    export_file_with_deps(
        &out_dir,
        "model/update-photo.ts",
        &[
            export::<TypeDto>(&ExportConfiguration::default())?,
            export::<UpdateModelPhotoDto>(&ExportConfiguration::default())?,
        ],
    )?;
    export_file_with_deps(
        &out_dir,
        "user/get-by-id.ts",
        &[export::<GetByIdUserDto>(&ExportConfiguration::default())?],
    )?;
    export_file_with_deps(
        &out_dir,
        "user/sign-in.ts",
        &[export::<SignInUserDto>(&ExportConfiguration::default())?],
    )?;
    export_file_with_deps(
        &out_dir,
        "user/sign-up.ts",
        &[export::<SignUpUserDto>(&ExportConfiguration::default())?],
    )?;
    export_file_with_deps(
        &out_dir,
        "user/update.ts",
        &[export::<UpdateUserDto>(&ExportConfiguration::default())?],
    )?;
    export_file_with_deps(
        &out_dir,
        "user/update-photo.ts",
        &[export::<UpdateUserPhotoDto>(&ExportConfiguration::default())?],
    )?;
    export_file_with_deps(
        &out_dir,
        "session/create.ts",
        &[export::<CreateSessionDto>(&ExportConfiguration::default())?],
    )?;
    export_file_with_deps(
        &out_dir,
        "session/list.ts",
        &[
            export::<TypeDto>(&ExportConfiguration::default())?,
            export::<RatingDto>(&ExportConfiguration::default())?,
            export::<ListPerformedVariationDto>(&ExportConfiguration::default())?,
            export::<ListSessionDto>(&ExportConfiguration::default())?,
        ],
    )?;
    export_file_with_deps(
        &out_dir,
        "session/update.ts",
        &[export::<UpdateSessionDto>(&ExportConfiguration::default())?],
    )?;
    export_file_with_deps(
        &out_dir,
        "session/add-performed-variation.ts",
        &[
            export::<RatingDto>(&ExportConfiguration::default())?,
            export::<AddPerformedVariationDto>(&ExportConfiguration::default())?,
        ],
    )?;
    export_file_with_deps(
        &out_dir,
        "session/update-performed-variation.ts",
        &[export::<RatingDto>(&ExportConfiguration::default())?],
    )?;
    export_file_with_deps(
        &out_dir,
        "asset/photo.ts",
        &[export::<PhotoPathsDto>(&ExportConfiguration::default())?],
    )?;
    export_file_with_deps(
        &out_dir,
        "asset/video.ts",
        &[export::<VideoPathsDto>(&ExportConfiguration::default())?],
    )?;

    Ok(())
}

fn output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../frontend/src/models/__generated")
}

fn export_file_with_deps(
    out_dir: &std::path::Path,
    relative_path: &str,
    exports: &[String],
) -> Result<(), Box<dyn Error>> {
    write_type_file(
        out_dir.join(relative_path),
        "// @generated by cargo run -p rc-log-api --bin typegen_models\n// Do not edit manually.\n\n",
        exports.join("\n\n"),
    )
}

fn write_type_file(path: PathBuf, header: &str, body: String) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, format!("{header}{body}\n"))?;
    Ok(())
}
