use uuid::Uuid;

use crate::meta;

pub mod auth;
pub mod request;

pub mod cluster_rpc {
    tonic::include_proto!("cluster");
}

impl From<meta::search::Request> for cluster_rpc::SearchRequest {
    fn from(req: meta::search::Request) -> Self {
        let req_query = cluster_rpc::SearchQuery {
            sql: req.query.sql.clone(),
            sql_mode: req.query.sql_mode.clone(),
            from: req.query.from as i32,
            size: req.query.size as i32,
            start_time: req.query.start_time,
            end_time: req.query.end_time,
            track_total_hits: req.query.track_total_hits,
        };

        let job = cluster_rpc::Job {
            session_id: Uuid::new_v4().to_string(),
            job: "".to_string(),
            stage: 0,
            partition: 0,
        };

        let mut aggs = Vec::new();
        for (name, sql) in req.aggs {
            aggs.push(cluster_rpc::SearchAggRequest { name, sql });
        }

        cluster_rpc::SearchRequest {
            job: Some(job),
            org_id: "".to_string(),
            stype: cluster_rpc::SearchType::User.into(),
            query: Some(req_query),
            aggs,
            partition: None,
            file_list: vec![],
            stream_type: "".to_string(),
        }
    }
}

impl From<&meta::common::FileMeta> for cluster_rpc::FileMeta {
    fn from(req: &meta::common::FileMeta) -> Self {
        cluster_rpc::FileMeta {
            min_ts: req.min_ts,
            max_ts: req.max_ts,
            records: req.records,
            original_size: req.original_size,
            compressed_size: req.compressed_size,
        }
    }
}

impl From<&cluster_rpc::FileMeta> for meta::common::FileMeta {
    fn from(req: &cluster_rpc::FileMeta) -> Self {
        meta::common::FileMeta {
            min_ts: req.min_ts,
            max_ts: req.max_ts,
            records: req.records,
            original_size: req.original_size,
            compressed_size: req.compressed_size,
        }
    }
}

impl From<&meta::common::FileKey> for cluster_rpc::FileKey {
    fn from(req: &meta::common::FileKey) -> Self {
        cluster_rpc::FileKey {
            key: req.key.clone(),
            meta: Some(cluster_rpc::FileMeta::from(&req.meta)),
            deleted: req.deleted,
        }
    }
}