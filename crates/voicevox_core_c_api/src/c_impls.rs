use std::{ffi::CString, path::Path, sync::Arc};

use voicevox_core::{InitializeOptions, OpenJtalk, Result, Synthesizer, VoiceModel, VoiceModelId};

use crate::{CApiResult, OpenJtalkRc, VoicevoxSynthesizer, VoicevoxVoiceModel};

impl OpenJtalkRc {
    pub(crate) fn new_with_initialize(open_jtalk_dic_dir: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            open_jtalk: Arc::new(OpenJtalk::new_with_initialize(open_jtalk_dic_dir)?),
        })
    }
}

impl VoicevoxSynthesizer {
    pub(crate) async fn new_with_initialize(
        open_jtalk: &OpenJtalkRc,
        options: &InitializeOptions,
    ) -> Result<Self> {
        let synthesizer =
            Synthesizer::new_with_initialize(open_jtalk.open_jtalk.clone(), options).await?;
        Ok(Self { synthesizer })
    }

    pub(crate) async fn load_voice_model(&self, model: &VoiceModel) -> CApiResult<()> {
        self.synthesizer.load_voice_model(model).await?;
        Ok(())
    }

    pub(crate) fn unload_voice_model(&self, model_id: &VoiceModelId) -> Result<()> {
        self.synthesizer.unload_voice_model(model_id)?;
        Ok(())
    }

    pub(crate) fn metas(&self) -> CString {
        let metas = &self.synthesizer.metas();
        CString::new(serde_json::to_string(metas).unwrap()).unwrap()
    }
}

impl VoicevoxVoiceModel {
    pub(crate) async fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let model = VoiceModel::from_path(path).await?;
        let id = CString::new(model.id().raw_voice_model_id().as_str()).unwrap();
        let metas = CString::new(serde_json::to_string(model.metas()).unwrap()).unwrap();
        Ok(Self { model, id, metas })
    }
}
