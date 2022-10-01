mod get_files_response;
mod update_file_response;
mod create_file_response;
mod file;

pub type GetFilesResponse = get_files_response::GetFilesResponse;
pub type CreateFilesResponse = create_file_response::CreateFilesResponse;
pub type UpdateFilesResponse = update_file_response::UpdateFilesResponse;
pub type File = file::File;
