use netflow_parser::netflow_common::NetflowCommonFlowSet;

#[derive(Default, Debug)]
pub struct Store {
    pub netflowsets: Vec<NetflowCommonFlowSet>,
}
