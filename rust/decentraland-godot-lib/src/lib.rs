/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;

pub mod dcl;
pub mod http_request;
pub mod realm;
pub mod scene_runner;

struct DecentralandGodotLibrary;

#[gdextension]
unsafe impl ExtensionLibrary for DecentralandGodotLibrary {}
