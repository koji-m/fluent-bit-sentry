extern crate fluentbit;
use fluentbit::*;

extern crate rmpv;
extern crate serde_json;

extern crate serde;


#[derive(Default)]
struct OutputSentry {}

impl FLBPluginMethods for OutputSentry {
    
    fn plugin_register(&mut self, info: &mut PluginInfo) -> FLBResult{
        info.name = "sentry".into();
        info.description = "Sentry Output Plugin".into();
        Ok(())
    }

    fn plugin_init(&mut self) -> FLBResult{
        Ok(())
    }

    fn plugin_flush(&mut self, data: &[u8]) -> FLBResult{
        Ok(())
    }

    fn plugin_exit(&mut self) -> FLBResult{
        Ok(())
    }
    
}

create_boilerplate!(OutputSentry::default());