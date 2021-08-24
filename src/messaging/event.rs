/*
Copyright 2021 Gonzalo Fernández Hernández

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use crate::ValueId;
use crate::CommandId;

/// Class that represents an event from the UI to the app
pub enum Event {
    RequestCommand(CommandId),
    Clicked(CommandId),

    RequestValue(ValueId),
    StateChanged(ValueId, bool),
    NumberInserted(ValueId, i32),
    FloatInserted(ValueId, f32),
    TextInserted(ValueId, String),

    Quit()
}