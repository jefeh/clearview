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

use crate::ScreenId;
use crate::CommandId;
use crate::ValueId;

pub enum ValueState {
    Enabled,
    Correct,
    Wrong,
    Disabled,
    Hidden
}

pub enum CommandState {
    Enabled,
    Disabled,
    Hidden
}

/// Class that represents a message from the App to the UI
pub enum Message {
    ChangeScreen(ScreenId),
    PushScreen(ScreenId),
    PopScreen(),
    ClearScreenStack(),

    CommandAvailable(CommandId, ValueState),
    ValueChangedState(ValueId, bool, ValueState), 
    ValueChangedNumber(ValueId, i32, ValueState),
    ValueChangedFloat(ValueId, f32, ValueState),
    ValueChangedText(ValueId, String, ValueState)
}