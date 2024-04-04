use domain::models::EgressDestination;
use livekit_protocol::direct_file_output::Output as DirectFileOutputOptions;
use livekit_protocol::egress_info::Request;
use livekit_protocol::track_egress_request::Output;
pub fn get_track_egress_destination(request: Option<Request>) -> Option<EgressDestination> {
    request.as_ref()?;
    if let Request::Track(req) = request.unwrap() {
        match req.output {
            Some(output) => {
                // do something
                match output {
                    Output::File(f) => {
                        let destination = f.output;
                        match destination {
                            Some(d) => {
                                match d {
                                    DirectFileOutputOptions::S3(_) => Some(EgressDestination::S3),
                                    // FixMe: Other Options not supported yet by the deployment
                                    _ => None,
                                }
                            }
                            None => Some(EgressDestination::LocalFile),
                        }
                    }
                    _ => None,
                }
            }
            None => None,
        }
    } else {
        None
    }
}

pub fn get_track_egress_destination_path(
    result: Option<livekit_protocol::egress_info::Result>,
) -> Option<String> {
    if let Some(result) = result {
        if let livekit_protocol::egress_info::Result::File(f) = result {
            let filename = f.filename;
            let path = f.location;
            Some(format!("{}/{}", path, filename))
        } else {
            None
        }
    } else {
        None
    }
}
