use crate::datatypes::{
    AffineTransform, MaterialHandle, MeshHandle, ModelVertex, ObjectHandle, TextureFormat, TextureHandle,
};
use parking_lot::RwLock;
use smallvec::SmallVec;
use std::mem;

pub enum SceneChangeInstruction {
    AddMesh {
        handle: MeshHandle,
        vertices: Vec<ModelVertex>,
        indices: Vec<u32>,
        material_count: u32,
        // TODO: Bones/joints/animation
    },
    RemoveMesh {
        mesh: MeshHandle,
    },
    AddTexture {
        handle: TextureHandle,
        data: Vec<u8>,
        format: TextureFormat,
        width: u32,
        height: u32,
    },
    RemoveTexture {
        texture: TextureHandle,
    },
    AddMaterial {
        // Consider:
        //
        // - albedo and opacity
        // - normal
        // - roughness
        // - specular color
        // - thiccness for leaves
        // - porosity, so I can do things like make things wet when it rains
        // - Maybe subsurface scattering radii? Or some kind of transmission value
        // - Index of Refraction for transparent things
        handle: MaterialHandle,
        color: Option<TextureHandle>,
        normal: Option<TextureHandle>,
        roughness: Option<TextureHandle>,
        specular: Option<TextureHandle>,
    },
    RemoveMaterial {
        material: MaterialHandle,
    },
    AddObject {
        handle: ObjectHandle,
        mesh: MeshHandle,
        materials: SmallVec<[MaterialHandle; 4]>,
        transform: AffineTransform,
    },
    SetObjectTransform {
        object: ObjectHandle,
        transform: AffineTransform,
    },
    RemoveObject {
        object: ObjectHandle,
    },
}

pub enum GeneralInstruction {}

pub struct InstructionStream {
    pub scene_change: RwLock<Vec<SceneChangeInstruction>>,
    pub general: RwLock<Vec<SceneChangeInstruction>>,
}
impl InstructionStream {
    pub fn new() -> Self {
        Self {
            scene_change: RwLock::new(Vec::new()),
            general: RwLock::new(Vec::new()),
        }
    }
}

pub struct InstructionStreamPair {
    pub producer: InstructionStream,
    pub consumer: InstructionStream,
}
impl InstructionStreamPair {
    pub fn new() -> Self {
        Self {
            producer: InstructionStream::new(),
            consumer: InstructionStream::new(),
        }
    }

    pub fn swap(&self) {
        let mut scene_produce = self.producer.scene_change.write();
        let mut scene_consume = self.consumer.scene_change.write();

        let mut general_produce = self.producer.general.write();
        let mut general_consume = self.consumer.general.write();

        scene_produce.clear();
        general_produce.clear();

        mem::swap(&mut *scene_produce, &mut *scene_consume);
        mem::swap(&mut *general_produce, &mut *general_consume);
    }
}
