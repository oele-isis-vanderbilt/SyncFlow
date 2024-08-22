use domain::models::EgressDestination;
use livekit_protocol::direct_file_output::Output as DirectFileOutputOptions;
use livekit_protocol::egress_info::Request;
use livekit_protocol::track_egress_request::Output;

pub fn get_track_egress_destination(request: Option<Request>) -> Option<EgressDestination> {
    if let Some(Request::Track(req)) = request {
        req.output.and_then(|output| match output {
            Output::File(f) => {
                f.output
                    .and_then(|destination| match destination {
                        DirectFileOutputOptions::S3(_) => Some(EgressDestination::S3),
                        // FixMe: Other Options not supported yet by the deployment
                        _ => None,
                    })
                    .or(Some(EgressDestination::LocalFile))
            }
            _ => None,
        })
    } else {
        None
    }
}

pub fn get_track_egress_destination_path(
    result: Option<livekit_protocol::egress_info::Result>,
) -> Option<String> {
    result.and_then(|result| {
        if let livekit_protocol::egress_info::Result::File(f) = result {
            Some(format!("{}/{}", f.location, f.filename))
        } else {
            None
        }
    })
}
