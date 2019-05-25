pub mod input { // Windows.Gaming.Input
use crate::prelude::*;
DEFINE_IID!(IID_IArcadeStick, 2974438301, 48891, 19585, 128, 81, 21, 236, 243, 177, 48, 54);
RT_INTERFACE!{interface IArcadeStick(IArcadeStickVtbl): IInspectable(IInspectableVtbl) [IID_IArcadeStick] {
    fn GetButtonLabel(&self, button: ArcadeStickButtons, out: *mut GameControllerButtonLabel) -> HRESULT,
    fn GetCurrentReading(&self, out: *mut ArcadeStickReading) -> HRESULT
}}
impl ComPtr<IArcadeStick> {
    #[inline] pub fn get_button_label(&self, button: ArcadeStickButtons) -> Result<GameControllerButtonLabel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetButtonLabel)(self.deref() as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_reading(&self) -> Result<ArcadeStickReading> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetCurrentReading)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ArcadeStick: IArcadeStick}
impl RtActivatable<IArcadeStickStatics> for ArcadeStick {}
impl RtActivatable<IArcadeStickStatics2> for ArcadeStick {}
impl ArcadeStick {
    #[inline] pub fn add_arcade_stick_added(value: &ComPtr<foundation::EventHandler<ArcadeStick>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IArcadeStickStatics>>::get_activation_factory().add_arcade_stick_added(value)
    }
    #[inline] pub fn remove_arcade_stick_added(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IArcadeStickStatics>>::get_activation_factory().remove_arcade_stick_added(token)
    }
    #[inline] pub fn add_arcade_stick_removed(value: &ComPtr<foundation::EventHandler<ArcadeStick>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IArcadeStickStatics>>::get_activation_factory().add_arcade_stick_removed(value)
    }
    #[inline] pub fn remove_arcade_stick_removed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IArcadeStickStatics>>::get_activation_factory().remove_arcade_stick_removed(token)
    }
    #[inline] pub fn get_arcade_sticks() -> Result<Option<ComPtr<foundation::collections::IVectorView<ArcadeStick>>>> {
        <Self as RtActivatable<IArcadeStickStatics>>::get_activation_factory().get_arcade_sticks()
    }
    #[inline] pub fn from_game_controller(gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<ArcadeStick>>> {
        <Self as RtActivatable<IArcadeStickStatics2>>::get_activation_factory().from_game_controller(gameController)
    }
}
DEFINE_CLSID!(ArcadeStick(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,65,114,99,97,100,101,83,116,105,99,107,0]) [CLSID_ArcadeStick]);
RT_ENUM! { enum ArcadeStickButtons: u32 {
    None = 0, StickUp = 1, StickDown = 2, StickLeft = 4, StickRight = 8, Action1 = 16, Action2 = 32, Action3 = 64, Action4 = 128, Action5 = 256, Action6 = 512, Special1 = 1024, Special2 = 2048,
}}
RT_STRUCT! { struct ArcadeStickReading {
    Timestamp: u64, Buttons: ArcadeStickButtons,
}}
DEFINE_IID!(IID_IArcadeStickStatics, 1547155656, 14257, 19160, 148, 88, 32, 15, 26, 48, 1, 142);
RT_INTERFACE!{static interface IArcadeStickStatics(IArcadeStickStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IArcadeStickStatics] {
    fn add_ArcadeStickAdded(&self, value: *mut foundation::EventHandler<ArcadeStick>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ArcadeStickAdded(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_ArcadeStickRemoved(&self, value: *mut foundation::EventHandler<ArcadeStick>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ArcadeStickRemoved(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_ArcadeSticks(&self, out: *mut *mut foundation::collections::IVectorView<ArcadeStick>) -> HRESULT
}}
impl ComPtr<IArcadeStickStatics> {
    #[inline] pub fn add_arcade_stick_added(&self, value: &ComPtr<foundation::EventHandler<ArcadeStick>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_ArcadeStickAdded)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_arcade_stick_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_ArcadeStickAdded)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_arcade_stick_removed(&self, value: &ComPtr<foundation::EventHandler<ArcadeStick>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_ArcadeStickRemoved)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_arcade_stick_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_ArcadeStickRemoved)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_arcade_sticks(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<ArcadeStick>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ArcadeSticks)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IArcadeStickStatics2, 1387648836, 48006, 17498, 181, 156, 89, 111, 14, 42, 73, 223);
RT_INTERFACE!{static interface IArcadeStickStatics2(IArcadeStickStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IArcadeStickStatics2] {
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut ArcadeStick) -> HRESULT
}}
impl ComPtr<IArcadeStickStatics2> {
    #[inline] pub fn from_game_controller(&self, gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<ArcadeStick>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).FromGameController)(self.deref() as *const _ as *mut _, gameController.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IFlightStick, 3030564892, 47163, 17497, 161, 169, 151, 176, 60, 51, 218, 124);
RT_INTERFACE!{interface IFlightStick(IFlightStickVtbl): IInspectable(IInspectableVtbl) [IID_IFlightStick] {
    fn get_HatSwitchKind(&self, out: *mut GameControllerSwitchKind) -> HRESULT,
    fn GetButtonLabel(&self, button: FlightStickButtons, out: *mut GameControllerButtonLabel) -> HRESULT,
    fn GetCurrentReading(&self, out: *mut FlightStickReading) -> HRESULT
}}
impl ComPtr<IFlightStick> {
    #[inline] pub fn get_hat_switch_kind(&self) -> Result<GameControllerSwitchKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_HatSwitchKind)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_button_label(&self, button: FlightStickButtons) -> Result<GameControllerButtonLabel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetButtonLabel)(self.deref() as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_reading(&self) -> Result<FlightStickReading> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetCurrentReading)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class FlightStick: IFlightStick}
impl RtActivatable<IFlightStickStatics> for FlightStick {}
impl FlightStick {
    #[inline] pub fn add_flight_stick_added(value: &ComPtr<foundation::EventHandler<FlightStick>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().add_flight_stick_added(value)
    }
    #[inline] pub fn remove_flight_stick_added(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().remove_flight_stick_added(token)
    }
    #[inline] pub fn add_flight_stick_removed(value: &ComPtr<foundation::EventHandler<FlightStick>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().add_flight_stick_removed(value)
    }
    #[inline] pub fn remove_flight_stick_removed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().remove_flight_stick_removed(token)
    }
    #[inline] pub fn get_flight_sticks() -> Result<Option<ComPtr<foundation::collections::IVectorView<FlightStick>>>> {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().get_flight_sticks()
    }
    #[inline] pub fn from_game_controller(gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<FlightStick>>> {
        <Self as RtActivatable<IFlightStickStatics>>::get_activation_factory().from_game_controller(gameController)
    }
}
DEFINE_CLSID!(FlightStick(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,70,108,105,103,104,116,83,116,105,99,107,0]) [CLSID_FlightStick]);
RT_ENUM! { enum FlightStickButtons: u32 {
    None = 0, FirePrimary = 1, FireSecondary = 2,
}}
RT_STRUCT! { struct FlightStickReading {
    Timestamp: u64, Buttons: FlightStickButtons, HatSwitch: GameControllerSwitchPosition, Roll: f64, Pitch: f64, Yaw: f64, Throttle: f64,
}}
DEFINE_IID!(IID_IFlightStickStatics, 1427411530, 65228, 17246, 131, 220, 92, 236, 138, 24, 165, 32);
RT_INTERFACE!{static interface IFlightStickStatics(IFlightStickStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IFlightStickStatics] {
    fn add_FlightStickAdded(&self, value: *mut foundation::EventHandler<FlightStick>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FlightStickAdded(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_FlightStickRemoved(&self, value: *mut foundation::EventHandler<FlightStick>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FlightStickRemoved(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_FlightSticks(&self, out: *mut *mut foundation::collections::IVectorView<FlightStick>) -> HRESULT,
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut FlightStick) -> HRESULT
}}
impl ComPtr<IFlightStickStatics> {
    #[inline] pub fn add_flight_stick_added(&self, value: &ComPtr<foundation::EventHandler<FlightStick>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_FlightStickAdded)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_flight_stick_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_FlightStickAdded)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_flight_stick_removed(&self, value: &ComPtr<foundation::EventHandler<FlightStick>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_FlightStickRemoved)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_flight_stick_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_FlightStickRemoved)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_flight_sticks(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<FlightStick>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_FlightSticks)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn from_game_controller(&self, gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<FlightStick>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).FromGameController)(self.deref() as *const _ as *mut _, gameController.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGameController, 464479522, 24420, 17093, 130, 103, 185, 254, 34, 21, 191, 189);
RT_INTERFACE!{interface IGameController(IGameControllerVtbl): IInspectable(IInspectableVtbl) [IID_IGameController] {
    fn add_HeadsetConnected(&self, value: *mut foundation::TypedEventHandler<IGameController, Headset>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_HeadsetConnected(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_HeadsetDisconnected(&self, value: *mut foundation::TypedEventHandler<IGameController, Headset>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_HeadsetDisconnected(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    #[cfg(feature="windows-system")] fn add_UserChanged(&self, value: *mut foundation::TypedEventHandler<IGameController, super::super::system::UserChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UserChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Headset(&self, out: *mut *mut Headset) -> HRESULT,
    fn get_IsWireless(&self, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut *mut super::super::system::User) -> HRESULT
}}
impl ComPtr<IGameController> {
    #[inline] pub fn add_headset_connected(&self, value: &ComPtr<foundation::TypedEventHandler<IGameController, Headset>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_HeadsetConnected)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_headset_connected(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_HeadsetConnected)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_headset_disconnected(&self, value: &ComPtr<foundation::TypedEventHandler<IGameController, Headset>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_HeadsetDisconnected)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_headset_disconnected(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_HeadsetDisconnected)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn add_user_changed(&self, value: &ComPtr<foundation::TypedEventHandler<IGameController, super::super::system::UserChangedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_UserChanged)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_user_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_UserChanged)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_headset(&self) -> Result<Option<ComPtr<Headset>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Headset)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_wireless(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsWireless)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<super::super::system::User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_User)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGameControllerBatteryInfo, 3706504833, 14691, 19878, 149, 93, 85, 63, 59, 111, 97, 97);
RT_INTERFACE!{interface IGameControllerBatteryInfo(IGameControllerBatteryInfoVtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerBatteryInfo] {
    #[cfg(feature="windows-devices")] fn TryGetBatteryReport(&self, out: *mut *mut super::super::devices::power::BatteryReport) -> HRESULT
}}
impl ComPtr<IGameControllerBatteryInfo> {
    #[cfg(feature="windows-devices")] #[inline] pub fn try_get_battery_report(&self) -> Result<Option<ComPtr<super::super::devices::power::BatteryReport>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).TryGetBatteryReport)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum GameControllerButtonLabel: i32 {
    None = 0, XboxBack = 1, XboxStart = 2, XboxMenu = 3, XboxView = 4, XboxUp = 5, XboxDown = 6, XboxLeft = 7, XboxRight = 8, XboxA = 9, XboxB = 10, XboxX = 11, XboxY = 12, XboxLeftBumper = 13, XboxLeftTrigger = 14, XboxLeftStickButton = 15, XboxRightBumper = 16, XboxRightTrigger = 17, XboxRightStickButton = 18, XboxPaddle1 = 19, XboxPaddle2 = 20, XboxPaddle3 = 21, XboxPaddle4 = 22, Mode = 23, Select = 24, Menu = 25, View = 26, Back = 27, Start = 28, Options = 29, Share = 30, Up = 31, Down = 32, Left = 33, Right = 34, LetterA = 35, LetterB = 36, LetterC = 37, LetterL = 38, LetterR = 39, LetterX = 40, LetterY = 41, LetterZ = 42, Cross = 43, Circle = 44, Square = 45, Triangle = 46, LeftBumper = 47, LeftTrigger = 48, LeftStickButton = 49, Left1 = 50, Left2 = 51, Left3 = 52, RightBumper = 53, RightTrigger = 54, RightStickButton = 55, Right1 = 56, Right2 = 57, Right3 = 58, Paddle1 = 59, Paddle2 = 60, Paddle3 = 61, Paddle4 = 62, Plus = 63, Minus = 64, DownLeftArrow = 65, DialLeft = 66, DialRight = 67, Suspension = 68,
}}
RT_ENUM! { enum GameControllerSwitchKind: i32 {
    TwoWay = 0, FourWay = 1, EightWay = 2,
}}
RT_ENUM! { enum GameControllerSwitchPosition: i32 {
    Center = 0, Up = 1, UpRight = 2, Right = 3, DownRight = 4, Down = 5, DownLeft = 6, Left = 7, UpLeft = 8,
}}
DEFINE_IID!(IID_IGamepad, 3162223676, 2665, 14595, 158, 157, 165, 15, 134, 164, 93, 229);
RT_INTERFACE!{interface IGamepad(IGamepadVtbl): IInspectable(IInspectableVtbl) [IID_IGamepad] {
    fn get_Vibration(&self, out: *mut GamepadVibration) -> HRESULT,
    fn put_Vibration(&self, value: GamepadVibration) -> HRESULT,
    fn GetCurrentReading(&self, out: *mut GamepadReading) -> HRESULT
}}
impl ComPtr<IGamepad> {
    #[inline] pub fn get_vibration(&self) -> Result<GamepadVibration> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Vibration)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_vibration(&self, value: GamepadVibration) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Vibration)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_current_reading(&self) -> Result<GamepadReading> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetCurrentReading)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Gamepad: IGamepad}
impl RtActivatable<IGamepadStatics> for Gamepad {}
impl RtActivatable<IGamepadStatics2> for Gamepad {}
impl Gamepad {
    #[inline] pub fn add_gamepad_added(value: &ComPtr<foundation::EventHandler<Gamepad>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IGamepadStatics>>::get_activation_factory().add_gamepad_added(value)
    }
    #[inline] pub fn remove_gamepad_added(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IGamepadStatics>>::get_activation_factory().remove_gamepad_added(token)
    }
    #[inline] pub fn add_gamepad_removed(value: &ComPtr<foundation::EventHandler<Gamepad>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IGamepadStatics>>::get_activation_factory().add_gamepad_removed(value)
    }
    #[inline] pub fn remove_gamepad_removed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IGamepadStatics>>::get_activation_factory().remove_gamepad_removed(token)
    }
    #[inline] pub fn get_gamepads() -> Result<Option<ComPtr<foundation::collections::IVectorView<Gamepad>>>> {
        <Self as RtActivatable<IGamepadStatics>>::get_activation_factory().get_gamepads()
    }
    #[inline] pub fn from_game_controller(gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<Gamepad>>> {
        <Self as RtActivatable<IGamepadStatics2>>::get_activation_factory().from_game_controller(gameController)
    }
}
DEFINE_CLSID!(Gamepad(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,71,97,109,101,112,97,100,0]) [CLSID_Gamepad]);
DEFINE_IID!(IID_IGamepad2, 1008110013, 22805, 16965, 176, 192, 200, 159, 174, 3, 8, 255);
RT_INTERFACE!{interface IGamepad2(IGamepad2Vtbl): IInspectable(IInspectableVtbl) [IID_IGamepad2] {
    fn GetButtonLabel(&self, button: GamepadButtons, out: *mut GameControllerButtonLabel) -> HRESULT
}}
impl ComPtr<IGamepad2> {
    #[inline] pub fn get_button_label(&self, button: GamepadButtons) -> Result<GameControllerButtonLabel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetButtonLabel)(self.deref() as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum GamepadButtons: u32 {
    None = 0, Menu = 1, View = 2, A = 4, B = 8, X = 16, Y = 32, DPadUp = 64, DPadDown = 128, DPadLeft = 256, DPadRight = 512, LeftShoulder = 1024, RightShoulder = 2048, LeftThumbstick = 4096, RightThumbstick = 8192, Paddle1 = 16384, Paddle2 = 32768, Paddle3 = 65536, Paddle4 = 131072,
}}
RT_STRUCT! { struct GamepadReading {
    Timestamp: u64, Buttons: GamepadButtons, LeftTrigger: f64, RightTrigger: f64, LeftThumbstickX: f64, LeftThumbstickY: f64, RightThumbstickX: f64, RightThumbstickY: f64,
}}
DEFINE_IID!(IID_IGamepadStatics, 2344412457, 54428, 14825, 149, 96, 228, 125, 222, 150, 183, 200);
RT_INTERFACE!{static interface IGamepadStatics(IGamepadStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGamepadStatics] {
    fn add_GamepadAdded(&self, value: *mut foundation::EventHandler<Gamepad>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GamepadAdded(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_GamepadRemoved(&self, value: *mut foundation::EventHandler<Gamepad>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GamepadRemoved(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Gamepads(&self, out: *mut *mut foundation::collections::IVectorView<Gamepad>) -> HRESULT
}}
impl ComPtr<IGamepadStatics> {
    #[inline] pub fn add_gamepad_added(&self, value: &ComPtr<foundation::EventHandler<Gamepad>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_GamepadAdded)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_gamepad_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_GamepadAdded)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_gamepad_removed(&self, value: &ComPtr<foundation::EventHandler<Gamepad>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_GamepadRemoved)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_gamepad_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_GamepadRemoved)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_gamepads(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<Gamepad>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Gamepads)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGamepadStatics2, 1114074565, 2134, 18372, 146, 19, 179, 149, 80, 76, 58, 60);
RT_INTERFACE!{static interface IGamepadStatics2(IGamepadStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IGamepadStatics2] {
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut Gamepad) -> HRESULT
}}
impl ComPtr<IGamepadStatics2> {
    #[inline] pub fn from_game_controller(&self, gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<Gamepad>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).FromGameController)(self.deref() as *const _ as *mut _, gameController.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_STRUCT! { struct GamepadVibration {
    LeftMotor: f64, RightMotor: f64, LeftTrigger: f64, RightTrigger: f64,
}}
DEFINE_IID!(IID_IHeadset, 1070683887, 26917, 16296, 145, 129, 2, 156, 82, 35, 174, 59);
RT_INTERFACE!{interface IHeadset(IHeadsetVtbl): IInspectable(IInspectableVtbl) [IID_IHeadset] {
    fn get_CaptureDeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RenderDeviceId(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IHeadset> {
    #[inline] pub fn get_capture_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_CaptureDeviceId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_render_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_RenderDeviceId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Headset: IHeadset}
RT_ENUM! { enum OptionalUINavigationButtons: u32 {
    None = 0, Context1 = 1, Context2 = 2, Context3 = 4, Context4 = 8, PageUp = 16, PageDown = 32, PageLeft = 64, PageRight = 128, ScrollUp = 256, ScrollDown = 512, ScrollLeft = 1024, ScrollRight = 2048,
}}
DEFINE_IID!(IID_IRacingWheel, 4115031407, 57606, 19586, 169, 15, 85, 64, 18, 144, 75, 133);
RT_INTERFACE!{interface IRacingWheel(IRacingWheelVtbl): IInspectable(IInspectableVtbl) [IID_IRacingWheel] {
    fn get_HasClutch(&self, out: *mut bool) -> HRESULT,
    fn get_HasHandbrake(&self, out: *mut bool) -> HRESULT,
    fn get_HasPatternShifter(&self, out: *mut bool) -> HRESULT,
    fn get_MaxPatternShifterGear(&self, out: *mut i32) -> HRESULT,
    fn get_MaxWheelAngle(&self, out: *mut f64) -> HRESULT,
    fn get_WheelMotor(&self, out: *mut *mut forcefeedback::ForceFeedbackMotor) -> HRESULT,
    fn GetButtonLabel(&self, button: RacingWheelButtons, out: *mut GameControllerButtonLabel) -> HRESULT,
    fn GetCurrentReading(&self, out: *mut RacingWheelReading) -> HRESULT
}}
impl ComPtr<IRacingWheel> {
    #[inline] pub fn get_has_clutch(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_HasClutch)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_has_handbrake(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_HasHandbrake)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_has_pattern_shifter(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_HasPatternShifter)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_pattern_shifter_gear(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_MaxPatternShifterGear)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_wheel_angle(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_MaxWheelAngle)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_wheel_motor(&self) -> Result<Option<ComPtr<forcefeedback::ForceFeedbackMotor>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_WheelMotor)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_button_label(&self, button: RacingWheelButtons) -> Result<GameControllerButtonLabel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetButtonLabel)(self.deref() as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_reading(&self) -> Result<RacingWheelReading> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetCurrentReading)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class RacingWheel: IRacingWheel}
impl RtActivatable<IRacingWheelStatics> for RacingWheel {}
impl RtActivatable<IRacingWheelStatics2> for RacingWheel {}
impl RacingWheel {
    #[inline] pub fn add_racing_wheel_added(value: &ComPtr<foundation::EventHandler<RacingWheel>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IRacingWheelStatics>>::get_activation_factory().add_racing_wheel_added(value)
    }
    #[inline] pub fn remove_racing_wheel_added(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IRacingWheelStatics>>::get_activation_factory().remove_racing_wheel_added(token)
    }
    #[inline] pub fn add_racing_wheel_removed(value: &ComPtr<foundation::EventHandler<RacingWheel>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IRacingWheelStatics>>::get_activation_factory().add_racing_wheel_removed(value)
    }
    #[inline] pub fn remove_racing_wheel_removed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IRacingWheelStatics>>::get_activation_factory().remove_racing_wheel_removed(token)
    }
    #[inline] pub fn get_racing_wheels() -> Result<Option<ComPtr<foundation::collections::IVectorView<RacingWheel>>>> {
        <Self as RtActivatable<IRacingWheelStatics>>::get_activation_factory().get_racing_wheels()
    }
    #[inline] pub fn from_game_controller(gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<RacingWheel>>> {
        <Self as RtActivatable<IRacingWheelStatics2>>::get_activation_factory().from_game_controller(gameController)
    }
}
DEFINE_CLSID!(RacingWheel(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,82,97,99,105,110,103,87,104,101,101,108,0]) [CLSID_RacingWheel]);
RT_ENUM! { enum RacingWheelButtons: u32 {
    None = 0, PreviousGear = 1, NextGear = 2, DPadUp = 4, DPadDown = 8, DPadLeft = 16, DPadRight = 32, Button1 = 64, Button2 = 128, Button3 = 256, Button4 = 512, Button5 = 1024, Button6 = 2048, Button7 = 4096, Button8 = 8192, Button9 = 16384, Button10 = 32768, Button11 = 65536, Button12 = 131072, Button13 = 262144, Button14 = 524288, Button15 = 1048576, Button16 = 2097152,
}}
RT_STRUCT! { struct RacingWheelReading {
    Timestamp: u64, Buttons: RacingWheelButtons, PatternShifterGear: i32, Wheel: f64, Throttle: f64, Brake: f64, Clutch: f64, Handbrake: f64,
}}
DEFINE_IID!(IID_IRacingWheelStatics, 985738453, 22555, 18742, 159, 148, 105, 241, 230, 81, 76, 125);
RT_INTERFACE!{static interface IRacingWheelStatics(IRacingWheelStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRacingWheelStatics] {
    fn add_RacingWheelAdded(&self, value: *mut foundation::EventHandler<RacingWheel>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RacingWheelAdded(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_RacingWheelRemoved(&self, value: *mut foundation::EventHandler<RacingWheel>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RacingWheelRemoved(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_RacingWheels(&self, out: *mut *mut foundation::collections::IVectorView<RacingWheel>) -> HRESULT
}}
impl ComPtr<IRacingWheelStatics> {
    #[inline] pub fn add_racing_wheel_added(&self, value: &ComPtr<foundation::EventHandler<RacingWheel>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_RacingWheelAdded)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_racing_wheel_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_RacingWheelAdded)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_racing_wheel_removed(&self, value: &ComPtr<foundation::EventHandler<RacingWheel>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_RacingWheelRemoved)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_racing_wheel_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_RacingWheelRemoved)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_racing_wheels(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<RacingWheel>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_RacingWheels)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRacingWheelStatics2, 3865492650, 60925, 17187, 169, 246, 60, 56, 64, 72, 209, 237);
RT_INTERFACE!{static interface IRacingWheelStatics2(IRacingWheelStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IRacingWheelStatics2] {
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut RacingWheel) -> HRESULT
}}
impl ComPtr<IRacingWheelStatics2> {
    #[inline] pub fn from_game_controller(&self, gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<RacingWheel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).FromGameController)(self.deref() as *const _ as *mut _, gameController.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRawGameController, 2091740561, 42977, 20337, 154, 120, 51, 233, 197, 223, 234, 98);
RT_INTERFACE!{interface IRawGameController(IRawGameControllerVtbl): IInspectable(IInspectableVtbl) [IID_IRawGameController] {
    fn get_AxisCount(&self, out: *mut i32) -> HRESULT,
    fn get_ButtonCount(&self, out: *mut i32) -> HRESULT,
    fn get_ForceFeedbackMotors(&self, out: *mut *mut foundation::collections::IVectorView<forcefeedback::ForceFeedbackMotor>) -> HRESULT,
    fn get_HardwareProductId(&self, out: *mut u16) -> HRESULT,
    fn get_HardwareVendorId(&self, out: *mut u16) -> HRESULT,
    fn get_SwitchCount(&self, out: *mut i32) -> HRESULT,
    fn GetButtonLabel(&self, buttonIndex: i32, out: *mut GameControllerButtonLabel) -> HRESULT,
    fn GetCurrentReading(&self, buttonArraySize: u32, buttonArray: *mut bool, switchArraySize: u32, switchArray: *mut GameControllerSwitchPosition, axisArraySize: u32, axisArray: *mut f64, out: *mut u64) -> HRESULT,
    fn GetSwitchKind(&self, switchIndex: i32, out: *mut GameControllerSwitchKind) -> HRESULT
}}
impl ComPtr<IRawGameController> {
    #[inline] pub fn get_axis_count(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_AxisCount)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_button_count(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ButtonCount)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_force_feedback_motors(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<forcefeedback::ForceFeedbackMotor>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ForceFeedbackMotors)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hardware_product_id(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_HardwareProductId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_hardware_vendor_id(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_HardwareVendorId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_switch_count(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_SwitchCount)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_button_label(&self, buttonIndex: i32) -> Result<GameControllerButtonLabel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetButtonLabel)(self.deref() as *const _ as *mut _, buttonIndex, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_reading(&self, buttonArray: &mut [bool], switchArray: &mut [GameControllerSwitchPosition], axisArray: &mut [f64]) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetCurrentReading)(self.deref() as *const _ as *mut _, buttonArray.len() as u32, buttonArray.as_mut_ptr() as *mut _, switchArray.len() as u32, switchArray.as_mut_ptr() as *mut _, axisArray.len() as u32, axisArray.as_mut_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_switch_kind(&self, switchIndex: i32) -> Result<GameControllerSwitchKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetSwitchKind)(self.deref() as *const _ as *mut _, switchIndex, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class RawGameController: IRawGameController}
impl RtActivatable<IRawGameControllerStatics> for RawGameController {}
impl RawGameController {
    #[inline] pub fn add_raw_game_controller_added(value: &ComPtr<foundation::EventHandler<RawGameController>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().add_raw_game_controller_added(value)
    }
    #[inline] pub fn remove_raw_game_controller_added(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().remove_raw_game_controller_added(token)
    }
    #[inline] pub fn add_raw_game_controller_removed(value: &ComPtr<foundation::EventHandler<RawGameController>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().add_raw_game_controller_removed(value)
    }
    #[inline] pub fn remove_raw_game_controller_removed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().remove_raw_game_controller_removed(token)
    }
    #[inline] pub fn get_raw_game_controllers() -> Result<Option<ComPtr<foundation::collections::IVectorView<RawGameController>>>> {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().get_raw_game_controllers()
    }
    #[inline] pub fn from_game_controller(gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<RawGameController>>> {
        <Self as RtActivatable<IRawGameControllerStatics>>::get_activation_factory().from_game_controller(gameController)
    }
}
DEFINE_CLSID!(RawGameController(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,82,97,119,71,97,109,101,67,111,110,116,114,111,108,108,101,114,0]) [CLSID_RawGameController]);
DEFINE_IID!(IID_IRawGameController2, 1136705589, 47987, 18262, 167, 135, 62, 214, 190, 166, 23, 189);
RT_INTERFACE!{interface IRawGameController2(IRawGameController2Vtbl): IInspectable(IInspectableVtbl) [IID_IRawGameController2] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_SimpleHapticsControllers(&self, out: *mut *mut foundation::collections::IVectorView<super::super::devices::haptics::SimpleHapticsController>) -> HRESULT,
    fn get_NonRoamableId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IRawGameController2> {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_simple_haptics_controllers(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::super::devices::haptics::SimpleHapticsController>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_SimpleHapticsControllers)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_non_roamable_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_NonRoamableId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_DisplayName)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRawGameControllerStatics, 3951888274, 59738, 19225, 175, 199, 10, 89, 248, 191, 117, 158);
RT_INTERFACE!{static interface IRawGameControllerStatics(IRawGameControllerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRawGameControllerStatics] {
    fn add_RawGameControllerAdded(&self, value: *mut foundation::EventHandler<RawGameController>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RawGameControllerAdded(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_RawGameControllerRemoved(&self, value: *mut foundation::EventHandler<RawGameController>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RawGameControllerRemoved(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_RawGameControllers(&self, out: *mut *mut foundation::collections::IVectorView<RawGameController>) -> HRESULT,
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut RawGameController) -> HRESULT
}}
impl ComPtr<IRawGameControllerStatics> {
    #[inline] pub fn add_raw_game_controller_added(&self, value: &ComPtr<foundation::EventHandler<RawGameController>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_RawGameControllerAdded)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_raw_game_controller_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_RawGameControllerAdded)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_raw_game_controller_removed(&self, value: &ComPtr<foundation::EventHandler<RawGameController>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_RawGameControllerRemoved)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_raw_game_controller_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_RawGameControllerRemoved)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_raw_game_controllers(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<RawGameController>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_RawGameControllers)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn from_game_controller(&self, gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<RawGameController>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).FromGameController)(self.deref() as *const _ as *mut _, gameController.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum RequiredUINavigationButtons: u32 {
    None = 0, Menu = 1, View = 2, Accept = 4, Cancel = 8, Up = 16, Down = 32, Left = 64, Right = 128,
}}
DEFINE_IID!(IID_IUINavigationController, 3853447133, 62734, 19029, 140, 220, 211, 50, 41, 84, 129, 117);
RT_INTERFACE!{interface IUINavigationController(IUINavigationControllerVtbl): IInspectable(IInspectableVtbl) [IID_IUINavigationController] {
    fn GetCurrentReading(&self, out: *mut UINavigationReading) -> HRESULT,
    fn GetOptionalButtonLabel(&self, button: OptionalUINavigationButtons, out: *mut GameControllerButtonLabel) -> HRESULT,
    fn GetRequiredButtonLabel(&self, button: RequiredUINavigationButtons, out: *mut GameControllerButtonLabel) -> HRESULT
}}
impl ComPtr<IUINavigationController> {
    #[inline] pub fn get_current_reading(&self) -> Result<UINavigationReading> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetCurrentReading)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_optional_button_label(&self, button: OptionalUINavigationButtons) -> Result<GameControllerButtonLabel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetOptionalButtonLabel)(self.deref() as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_required_button_label(&self, button: RequiredUINavigationButtons) -> Result<GameControllerButtonLabel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetRequiredButtonLabel)(self.deref() as *const _ as *mut _, button, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class UINavigationController: IUINavigationController}
impl RtActivatable<IUINavigationControllerStatics> for UINavigationController {}
impl RtActivatable<IUINavigationControllerStatics2> for UINavigationController {}
impl UINavigationController {
    #[inline] pub fn add_ui_navigation_controller_added(value: &ComPtr<foundation::EventHandler<UINavigationController>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IUINavigationControllerStatics>>::get_activation_factory().add_ui_navigation_controller_added(value)
    }
    #[inline] pub fn remove_ui_navigation_controller_added(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IUINavigationControllerStatics>>::get_activation_factory().remove_ui_navigation_controller_added(token)
    }
    #[inline] pub fn add_ui_navigation_controller_removed(value: &ComPtr<foundation::EventHandler<UINavigationController>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IUINavigationControllerStatics>>::get_activation_factory().add_ui_navigation_controller_removed(value)
    }
    #[inline] pub fn remove_ui_navigation_controller_removed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IUINavigationControllerStatics>>::get_activation_factory().remove_ui_navigation_controller_removed(token)
    }
    #[inline] pub fn get_ui_navigation_controllers() -> Result<Option<ComPtr<foundation::collections::IVectorView<UINavigationController>>>> {
        <Self as RtActivatable<IUINavigationControllerStatics>>::get_activation_factory().get_ui_navigation_controllers()
    }
    #[inline] pub fn from_game_controller(gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<UINavigationController>>> {
        <Self as RtActivatable<IUINavigationControllerStatics2>>::get_activation_factory().from_game_controller(gameController)
    }
}
DEFINE_CLSID!(UINavigationController(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,85,73,78,97,118,105,103,97,116,105,111,110,67,111,110,116,114,111,108,108,101,114,0]) [CLSID_UINavigationController]);
DEFINE_IID!(IID_IUINavigationControllerStatics, 789877514, 63224, 19016, 141, 137, 148, 120, 108, 202, 12, 46);
RT_INTERFACE!{static interface IUINavigationControllerStatics(IUINavigationControllerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUINavigationControllerStatics] {
    fn add_UINavigationControllerAdded(&self, value: *mut foundation::EventHandler<UINavigationController>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UINavigationControllerAdded(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_UINavigationControllerRemoved(&self, value: *mut foundation::EventHandler<UINavigationController>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UINavigationControllerRemoved(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_UINavigationControllers(&self, out: *mut *mut foundation::collections::IVectorView<UINavigationController>) -> HRESULT
}}
impl ComPtr<IUINavigationControllerStatics> {
    #[inline] pub fn add_ui_navigation_controller_added(&self, value: &ComPtr<foundation::EventHandler<UINavigationController>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_UINavigationControllerAdded)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_ui_navigation_controller_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_UINavigationControllerAdded)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_ui_navigation_controller_removed(&self, value: &ComPtr<foundation::EventHandler<UINavigationController>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_UINavigationControllerRemoved)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_ui_navigation_controller_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_UINavigationControllerRemoved)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_ui_navigation_controllers(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<UINavigationController>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_UINavigationControllers)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IUINavigationControllerStatics2, 3771410659, 45579, 19211, 158, 212, 243, 213, 60, 236, 13, 228);
RT_INTERFACE!{static interface IUINavigationControllerStatics2(IUINavigationControllerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IUINavigationControllerStatics2] {
    fn FromGameController(&self, gameController: *mut IGameController, out: *mut *mut UINavigationController) -> HRESULT
}}
impl ComPtr<IUINavigationControllerStatics2> {
    #[inline] pub fn from_game_controller(&self, gameController: &ComPtr<IGameController>) -> Result<Option<ComPtr<UINavigationController>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).FromGameController)(self.deref() as *const _ as *mut _, gameController.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_STRUCT! { struct UINavigationReading {
    Timestamp: u64, RequiredButtons: RequiredUINavigationButtons, OptionalButtons: OptionalUINavigationButtons,
}}
pub mod custom { // Windows.Gaming.Input.Custom
use crate::prelude::*;
DEFINE_IID!(IID_ICustomGameControllerFactory, 1772138078, 30094, 19646, 172, 230, 98, 21, 95, 233, 18, 111);
RT_INTERFACE!{interface ICustomGameControllerFactory(ICustomGameControllerFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ICustomGameControllerFactory] {
    fn CreateGameController(&self, provider: *mut IGameControllerProvider, out: *mut *mut IInspectable) -> HRESULT,
    fn OnGameControllerAdded(&self, value: *mut super::IGameController) -> HRESULT,
    fn OnGameControllerRemoved(&self, value: *mut super::IGameController) -> HRESULT
}}
impl ComPtr<ICustomGameControllerFactory> {
    #[inline] pub fn create_game_controller(&self, provider: &ComPtr<IGameControllerProvider>) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateGameController)(self.deref() as *const _ as *mut _, provider.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn on_game_controller_added(&self, value: &ComPtr<super::IGameController>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).OnGameControllerAdded)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn on_game_controller_removed(&self, value: &ComPtr<super::IGameController>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).OnGameControllerRemoved)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{static class GameControllerFactoryManager}
impl RtActivatable<IGameControllerFactoryManagerStatics> for GameControllerFactoryManager {}
impl RtActivatable<IGameControllerFactoryManagerStatics2> for GameControllerFactoryManager {}
impl GameControllerFactoryManager {
    #[inline] pub fn register_custom_factory_for_gip_interface(factory: &ComPtr<ICustomGameControllerFactory>, interfaceId: Guid) -> Result<()> {
        <Self as RtActivatable<IGameControllerFactoryManagerStatics>>::get_activation_factory().register_custom_factory_for_gip_interface(factory, interfaceId)
    }
    #[inline] pub fn register_custom_factory_for_hardware_id(factory: &ComPtr<ICustomGameControllerFactory>, hardwareVendorId: u16, hardwareProductId: u16) -> Result<()> {
        <Self as RtActivatable<IGameControllerFactoryManagerStatics>>::get_activation_factory().register_custom_factory_for_hardware_id(factory, hardwareVendorId, hardwareProductId)
    }
    #[inline] pub fn register_custom_factory_for_xusb_type(factory: &ComPtr<ICustomGameControllerFactory>, xusbType: XusbDeviceType, xusbSubtype: XusbDeviceSubtype) -> Result<()> {
        <Self as RtActivatable<IGameControllerFactoryManagerStatics>>::get_activation_factory().register_custom_factory_for_xusb_type(factory, xusbType, xusbSubtype)
    }
    #[inline] pub fn try_get_factory_controller_from_game_controller(factory: &ComPtr<ICustomGameControllerFactory>, gameController: &ComPtr<super::IGameController>) -> Result<Option<ComPtr<super::IGameController>>> {
        <Self as RtActivatable<IGameControllerFactoryManagerStatics2>>::get_activation_factory().try_get_factory_controller_from_game_controller(factory, gameController)
    }
}
DEFINE_CLSID!(GameControllerFactoryManager(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,67,117,115,116,111,109,46,71,97,109,101,67,111,110,116,114,111,108,108,101,114,70,97,99,116,111,114,121,77,97,110,97,103,101,114,0]) [CLSID_GameControllerFactoryManager]);
DEFINE_IID!(IID_IGameControllerFactoryManagerStatics, 919299811, 53409, 18822, 162, 76, 64, 177, 55, 222, 186, 158);
RT_INTERFACE!{static interface IGameControllerFactoryManagerStatics(IGameControllerFactoryManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerFactoryManagerStatics] {
    fn RegisterCustomFactoryForGipInterface(&self, factory: *mut ICustomGameControllerFactory, interfaceId: Guid) -> HRESULT,
    fn RegisterCustomFactoryForHardwareId(&self, factory: *mut ICustomGameControllerFactory, hardwareVendorId: u16, hardwareProductId: u16) -> HRESULT,
    fn RegisterCustomFactoryForXusbType(&self, factory: *mut ICustomGameControllerFactory, xusbType: XusbDeviceType, xusbSubtype: XusbDeviceSubtype) -> HRESULT
}}
impl ComPtr<IGameControllerFactoryManagerStatics> {
    #[inline] pub fn register_custom_factory_for_gip_interface(&self, factory: &ComPtr<ICustomGameControllerFactory>, interfaceId: Guid) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).RegisterCustomFactoryForGipInterface)(self.deref() as *const _ as *mut _, factory.deref() as *const _ as *mut _, interfaceId);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn register_custom_factory_for_hardware_id(&self, factory: &ComPtr<ICustomGameControllerFactory>, hardwareVendorId: u16, hardwareProductId: u16) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).RegisterCustomFactoryForHardwareId)(self.deref() as *const _ as *mut _, factory.deref() as *const _ as *mut _, hardwareVendorId, hardwareProductId);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn register_custom_factory_for_xusb_type(&self, factory: &ComPtr<ICustomGameControllerFactory>, xusbType: XusbDeviceType, xusbSubtype: XusbDeviceSubtype) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).RegisterCustomFactoryForXusbType)(self.deref() as *const _ as *mut _, factory.deref() as *const _ as *mut _, xusbType, xusbSubtype);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGameControllerFactoryManagerStatics2, 3939391044, 6623, 16661, 179, 42, 39, 147, 226, 174, 163, 187);
RT_INTERFACE!{static interface IGameControllerFactoryManagerStatics2(IGameControllerFactoryManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerFactoryManagerStatics2] {
    fn TryGetFactoryControllerFromGameController(&self, factory: *mut ICustomGameControllerFactory, gameController: *mut super::IGameController, out: *mut *mut super::IGameController) -> HRESULT
}}
impl ComPtr<IGameControllerFactoryManagerStatics2> {
    #[inline] pub fn try_get_factory_controller_from_game_controller(&self, factory: &ComPtr<ICustomGameControllerFactory>, gameController: &ComPtr<super::IGameController>) -> Result<Option<ComPtr<super::IGameController>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).TryGetFactoryControllerFromGameController)(self.deref() as *const _ as *mut _, factory.deref() as *const _ as *mut _, gameController.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGameControllerInputSink, 536279330, 50752, 19576, 168, 32, 154, 113, 92, 85, 139, 203);
RT_INTERFACE!{interface IGameControllerInputSink(IGameControllerInputSinkVtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerInputSink] {
    fn OnInputResumed(&self, timestamp: u64) -> HRESULT,
    fn OnInputSuspended(&self, timestamp: u64) -> HRESULT
}}
impl ComPtr<IGameControllerInputSink> {
    #[inline] pub fn on_input_resumed(&self, timestamp: u64) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).OnInputResumed)(self.deref() as *const _ as *mut _, timestamp);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn on_input_suspended(&self, timestamp: u64) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).OnInputSuspended)(self.deref() as *const _ as *mut _, timestamp);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGameControllerProvider, 3872864642, 10646, 17753, 177, 108, 62, 87, 212, 110, 88, 214);
RT_INTERFACE!{interface IGameControllerProvider(IGameControllerProviderVtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerProvider] {
    fn get_FirmwareVersionInfo(&self, out: *mut GameControllerVersionInfo) -> HRESULT,
    fn get_HardwareProductId(&self, out: *mut u16) -> HRESULT,
    fn get_HardwareVendorId(&self, out: *mut u16) -> HRESULT,
    fn get_HardwareVersionInfo(&self, out: *mut GameControllerVersionInfo) -> HRESULT,
    fn get_IsConnected(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IGameControllerProvider> {
    #[inline] pub fn get_firmware_version_info(&self) -> Result<GameControllerVersionInfo> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_FirmwareVersionInfo)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_hardware_product_id(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_HardwareProductId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_hardware_vendor_id(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_HardwareVendorId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_hardware_version_info(&self) -> Result<GameControllerVersionInfo> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_HardwareVersionInfo)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_connected(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsConnected)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_STRUCT! { struct GameControllerVersionInfo {
    Major: u16, Minor: u16, Build: u16, Revision: u16,
}}
RT_STRUCT! { struct GipFirmwareUpdateProgress {
    PercentCompleted: f64, CurrentComponentId: u32,
}}
DEFINE_IID!(IID_IGipFirmwareUpdateResult, 1803111730, 34131, 17042, 142, 3, 225, 102, 81, 162, 248, 188);
RT_INTERFACE!{interface IGipFirmwareUpdateResult(IGipFirmwareUpdateResultVtbl): IInspectable(IInspectableVtbl) [IID_IGipFirmwareUpdateResult] {
    fn get_ExtendedErrorCode(&self, out: *mut u32) -> HRESULT,
    fn get_FinalComponentId(&self, out: *mut u32) -> HRESULT,
    fn get_Status(&self, out: *mut GipFirmwareUpdateStatus) -> HRESULT
}}
impl ComPtr<IGipFirmwareUpdateResult> {
    #[inline] pub fn get_extended_error_code(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ExtendedErrorCode)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_final_component_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_FinalComponentId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<GipFirmwareUpdateStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Status)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class GipFirmwareUpdateResult: IGipFirmwareUpdateResult}
RT_ENUM! { enum GipFirmwareUpdateStatus: i32 {
    Completed = 0, UpToDate = 1, Failed = 2,
}}
DEFINE_IID!(IID_IGipGameControllerInputSink, 2718993087, 2545, 17340, 161, 64, 128, 248, 153, 236, 54, 251);
RT_INTERFACE!{interface IGipGameControllerInputSink(IGipGameControllerInputSinkVtbl): IInspectable(IInspectableVtbl) [IID_IGipGameControllerInputSink] {
    fn OnKeyReceived(&self, timestamp: u64, keyCode: u8, isPressed: bool) -> HRESULT,
    fn OnMessageReceived(&self, timestamp: u64, messageClass: GipMessageClass, messageId: u8, sequenceId: u8, messageBufferSize: u32, messageBuffer: *mut u8) -> HRESULT
}}
impl ComPtr<IGipGameControllerInputSink> {
    #[inline] pub fn on_key_received(&self, timestamp: u64, keyCode: u8, isPressed: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).OnKeyReceived)(self.deref() as *const _ as *mut _, timestamp, keyCode, isPressed);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn on_message_received(&self, timestamp: u64, messageClass: GipMessageClass, messageId: u8, sequenceId: u8, messageBuffer: &[u8]) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).OnMessageReceived)(self.deref() as *const _ as *mut _, timestamp, messageClass, messageId, sequenceId, messageBuffer.len() as u32, messageBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGipGameControllerProvider, 3687783961, 6901, 17832, 191, 2, 160, 238, 80, 200, 35, 252);
RT_INTERFACE!{interface IGipGameControllerProvider(IGipGameControllerProviderVtbl): IInspectable(IInspectableVtbl) [IID_IGipGameControllerProvider] {
    fn SendMessage(&self, messageClass: GipMessageClass, messageId: u8, messageBufferSize: u32, messageBuffer: *mut u8) -> HRESULT,
    fn SendReceiveMessage(&self, messageClass: GipMessageClass, messageId: u8, requestMessageBufferSize: u32, requestMessageBuffer: *mut u8, responseMessageBufferSize: u32, responseMessageBuffer: *mut u8) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UpdateFirmwareAsync(&self, firmwareImage: *mut crate::windows::storage::streams::IInputStream, out: *mut *mut foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>) -> HRESULT
}}
impl ComPtr<IGipGameControllerProvider> {
    #[inline] pub fn send_message(&self, messageClass: GipMessageClass, messageId: u8, messageBuffer: &[u8]) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SendMessage)(self.deref() as *const _ as *mut _, messageClass, messageId, messageBuffer.len() as u32, messageBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn send_receive_message(&self, messageClass: GipMessageClass, messageId: u8, requestMessageBuffer: &[u8], responseMessageBuffer: &mut [u8]) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SendReceiveMessage)(self.deref() as *const _ as *mut _, messageClass, messageId, requestMessageBuffer.len() as u32, requestMessageBuffer.as_ptr() as *mut _, responseMessageBuffer.len() as u32, responseMessageBuffer.as_mut_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn update_firmware_async(&self, firmwareImage: &ComPtr<crate::windows::storage::streams::IInputStream>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).UpdateFirmwareAsync)(self.deref() as *const _ as *mut _, firmwareImage.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GipGameControllerProvider: IGipGameControllerProvider}
RT_ENUM! { enum GipMessageClass: i32 {
    Command = 0, LowLatency = 1, StandardLatency = 2,
}}
DEFINE_IID!(IID_IHidGameControllerInputSink, 4149527330, 6189, 16612, 161, 38, 252, 238, 79, 250, 30, 49);
RT_INTERFACE!{interface IHidGameControllerInputSink(IHidGameControllerInputSinkVtbl): IInspectable(IInspectableVtbl) [IID_IHidGameControllerInputSink] {
    fn OnInputReportReceived(&self, timestamp: u64, reportId: u8, reportBufferSize: u32, reportBuffer: *mut u8) -> HRESULT
}}
impl ComPtr<IHidGameControllerInputSink> {
    #[inline] pub fn on_input_report_received(&self, timestamp: u64, reportId: u8, reportBuffer: &[u8]) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).OnInputReportReceived)(self.deref() as *const _ as *mut _, timestamp, reportId, reportBuffer.len() as u32, reportBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHidGameControllerProvider, 2513320692, 44016, 19304, 160, 129, 59, 125, 231, 63, 240, 231);
RT_INTERFACE!{interface IHidGameControllerProvider(IHidGameControllerProviderVtbl): IInspectable(IInspectableVtbl) [IID_IHidGameControllerProvider] {
    fn get_UsageId(&self, out: *mut u16) -> HRESULT,
    fn get_UsagePage(&self, out: *mut u16) -> HRESULT,
    fn GetFeatureReport(&self, reportId: u8, reportBufferSize: u32, reportBuffer: *mut u8) -> HRESULT,
    fn SendFeatureReport(&self, reportId: u8, reportBufferSize: u32, reportBuffer: *mut u8) -> HRESULT,
    fn SendOutputReport(&self, reportId: u8, reportBufferSize: u32, reportBuffer: *mut u8) -> HRESULT
}}
impl ComPtr<IHidGameControllerProvider> {
    #[inline] pub fn get_usage_id(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_UsageId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_usage_page(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_UsagePage)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_feature_report(&self, reportId: u8, reportBuffer: &mut [u8]) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).GetFeatureReport)(self.deref() as *const _ as *mut _, reportId, reportBuffer.len() as u32, reportBuffer.as_mut_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn send_feature_report(&self, reportId: u8, reportBuffer: &[u8]) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SendFeatureReport)(self.deref() as *const _ as *mut _, reportId, reportBuffer.len() as u32, reportBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn send_output_report(&self, reportId: u8, reportBuffer: &[u8]) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SendOutputReport)(self.deref() as *const _ as *mut _, reportId, reportBuffer.len() as u32, reportBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HidGameControllerProvider: IHidGameControllerProvider}
RT_ENUM! { enum XusbDeviceSubtype: i32 {
    Unknown = 0, Gamepad = 1, ArcadePad = 2, ArcadeStick = 3, FlightStick = 4, Wheel = 5, Guitar = 6, GuitarAlternate = 7, GuitarBass = 8, DrumKit = 9, DancePad = 10,
}}
RT_ENUM! { enum XusbDeviceType: i32 {
    Unknown = 0, Gamepad = 1,
}}
DEFINE_IID!(IID_IXusbGameControllerInputSink, 2997624213, 28363, 17075, 138, 171, 2, 84, 1, 202, 71, 18);
RT_INTERFACE!{interface IXusbGameControllerInputSink(IXusbGameControllerInputSinkVtbl): IInspectable(IInspectableVtbl) [IID_IXusbGameControllerInputSink] {
    fn OnInputReceived(&self, timestamp: u64, reportId: u8, inputBufferSize: u32, inputBuffer: *mut u8) -> HRESULT
}}
impl ComPtr<IXusbGameControllerInputSink> {
    #[inline] pub fn on_input_received(&self, timestamp: u64, reportId: u8, inputBuffer: &[u8]) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).OnInputReceived)(self.deref() as *const _ as *mut _, timestamp, reportId, inputBuffer.len() as u32, inputBuffer.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXusbGameControllerProvider, 1848209899, 3835, 18612, 128, 139, 131, 118, 67, 178, 242, 22);
RT_INTERFACE!{interface IXusbGameControllerProvider(IXusbGameControllerProviderVtbl): IInspectable(IInspectableVtbl) [IID_IXusbGameControllerProvider] {
    fn SetVibration(&self, lowFrequencyMotorSpeed: f64, highFrequencyMotorSpeed: f64) -> HRESULT
}}
impl ComPtr<IXusbGameControllerProvider> {
    #[inline] pub fn set_vibration(&self, lowFrequencyMotorSpeed: f64, highFrequencyMotorSpeed: f64) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SetVibration)(self.deref() as *const _ as *mut _, lowFrequencyMotorSpeed, highFrequencyMotorSpeed);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class XusbGameControllerProvider: IXusbGameControllerProvider}
} // Windows.Gaming.Input.Custom
pub mod forcefeedback { // Windows.Gaming.Input.ForceFeedback
use crate::prelude::*;
DEFINE_IID!(IID_IConditionForceEffect, 852617832, 13973, 20073, 133, 192, 205, 25, 68, 24, 145, 64);
RT_INTERFACE!{interface IConditionForceEffect(IConditionForceEffectVtbl): IInspectable(IInspectableVtbl) [IID_IConditionForceEffect] {
    fn get_Kind(&self, out: *mut ConditionForceEffectKind) -> HRESULT,
    fn SetParameters(&self, direction: foundation::numerics::Vector3, positiveCoefficient: f32, negativeCoefficient: f32, maxPositiveMagnitude: f32, maxNegativeMagnitude: f32, deadZone: f32, bias: f32) -> HRESULT
}}
impl ComPtr<IConditionForceEffect> {
    #[inline] pub fn get_kind(&self) -> Result<ConditionForceEffectKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Kind)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_parameters(&self, direction: foundation::numerics::Vector3, positiveCoefficient: f32, negativeCoefficient: f32, maxPositiveMagnitude: f32, maxNegativeMagnitude: f32, deadZone: f32, bias: f32) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SetParameters)(self.deref() as *const _ as *mut _, direction, positiveCoefficient, negativeCoefficient, maxPositiveMagnitude, maxNegativeMagnitude, deadZone, bias);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ConditionForceEffect: IForceFeedbackEffect}
impl RtActivatable<IConditionForceEffectFactory> for ConditionForceEffect {}
impl ConditionForceEffect {
    #[inline] pub fn create_instance(effectKind: ConditionForceEffectKind) -> Result<ComPtr<ConditionForceEffect>> {
        <Self as RtActivatable<IConditionForceEffectFactory>>::get_activation_factory().create_instance(effectKind)
    }
}
DEFINE_CLSID!(ConditionForceEffect(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,70,111,114,99,101,70,101,101,100,98,97,99,107,46,67,111,110,100,105,116,105,111,110,70,111,114,99,101,69,102,102,101,99,116,0]) [CLSID_ConditionForceEffect]);
DEFINE_IID!(IID_IConditionForceEffectFactory, 2443809380, 6160, 20150, 167, 115, 191, 211, 184, 205, 219, 171);
RT_INTERFACE!{static interface IConditionForceEffectFactory(IConditionForceEffectFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IConditionForceEffectFactory] {
    fn CreateInstance(&self, effectKind: ConditionForceEffectKind, out: *mut *mut ConditionForceEffect) -> HRESULT
}}
impl ComPtr<IConditionForceEffectFactory> {
    #[inline] pub fn create_instance(&self, effectKind: ConditionForceEffectKind) -> Result<ComPtr<ConditionForceEffect>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateInstance)(self.deref() as *const _ as *mut _, effectKind, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum ConditionForceEffectKind: i32 {
    Spring = 0, Damper = 1, Inertia = 2, Friction = 3,
}}
DEFINE_IID!(IID_IConstantForceEffect, 2616852800, 62407, 16732, 176, 104, 15, 6, 135, 52, 188, 224);
RT_INTERFACE!{interface IConstantForceEffect(IConstantForceEffectVtbl): IInspectable(IInspectableVtbl) [IID_IConstantForceEffect] {
    fn SetParameters(&self, vector: foundation::numerics::Vector3, duration: foundation::TimeSpan) -> HRESULT,
    fn SetParametersWithEnvelope(&self, vector: foundation::numerics::Vector3, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: foundation::TimeSpan, attackDuration: foundation::TimeSpan, sustainDuration: foundation::TimeSpan, releaseDuration: foundation::TimeSpan, repeatCount: u32) -> HRESULT
}}
impl ComPtr<IConstantForceEffect> {
    #[inline] pub fn set_parameters(&self, vector: foundation::numerics::Vector3, duration: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SetParameters)(self.deref() as *const _ as *mut _, vector, duration);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_parameters_with_envelope(&self, vector: foundation::numerics::Vector3, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: foundation::TimeSpan, attackDuration: foundation::TimeSpan, sustainDuration: foundation::TimeSpan, releaseDuration: foundation::TimeSpan, repeatCount: u32) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SetParametersWithEnvelope)(self.deref() as *const _ as *mut _, vector, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ConstantForceEffect: IForceFeedbackEffect}
impl RtActivatable<IActivationFactory> for ConstantForceEffect {}
DEFINE_CLSID!(ConstantForceEffect(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,70,111,114,99,101,70,101,101,100,98,97,99,107,46,67,111,110,115,116,97,110,116,70,111,114,99,101,69,102,102,101,99,116,0]) [CLSID_ConstantForceEffect]);
DEFINE_IID!(IID_IForceFeedbackEffect, 2709502476, 10980, 18626, 128, 99, 234, 189, 7, 119, 203, 137);
RT_INTERFACE!{interface IForceFeedbackEffect(IForceFeedbackEffectVtbl): IInspectable(IInspectableVtbl) [IID_IForceFeedbackEffect] {
    fn get_Gain(&self, out: *mut f64) -> HRESULT,
    fn put_Gain(&self, value: f64) -> HRESULT,
    fn get_State(&self, out: *mut ForceFeedbackEffectState) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl ComPtr<IForceFeedbackEffect> {
    #[inline] pub fn get_gain(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Gain)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_gain(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Gain)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<ForceFeedbackEffectState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_State)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Start)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Stop)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum ForceFeedbackEffectAxes: u32 {
    None = 0, X = 1, Y = 2, Z = 4,
}}
RT_ENUM! { enum ForceFeedbackEffectState: i32 {
    Stopped = 0, Running = 1, Paused = 2, Faulted = 3,
}}
RT_ENUM! { enum ForceFeedbackLoadEffectResult: i32 {
    Succeeded = 0, EffectStorageFull = 1, EffectNotSupported = 2,
}}
DEFINE_IID!(IID_IForceFeedbackMotor, 2369601916, 42474, 17686, 128, 38, 43, 0, 247, 78, 246, 229);
RT_INTERFACE!{interface IForceFeedbackMotor(IForceFeedbackMotorVtbl): IInspectable(IInspectableVtbl) [IID_IForceFeedbackMotor] {
    fn get_AreEffectsPaused(&self, out: *mut bool) -> HRESULT,
    fn get_MasterGain(&self, out: *mut f64) -> HRESULT,
    fn put_MasterGain(&self, value: f64) -> HRESULT,
    fn get_IsEnabled(&self, out: *mut bool) -> HRESULT,
    fn get_SupportedAxes(&self, out: *mut ForceFeedbackEffectAxes) -> HRESULT,
    fn LoadEffectAsync(&self, effect: *mut IForceFeedbackEffect, out: *mut *mut foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>) -> HRESULT,
    fn PauseAllEffects(&self) -> HRESULT,
    fn ResumeAllEffects(&self) -> HRESULT,
    fn StopAllEffects(&self) -> HRESULT,
    fn TryDisableAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn TryEnableAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn TryResetAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn TryUnloadEffectAsync(&self, effect: *mut IForceFeedbackEffect, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl ComPtr<IForceFeedbackMotor> {
    #[inline] pub fn get_are_effects_paused(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_AreEffectsPaused)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_master_gain(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_MasterGain)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_master_gain(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MasterGain)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsEnabled)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_supported_axes(&self) -> Result<ForceFeedbackEffectAxes> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_SupportedAxes)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn load_effect_async(&self, effect: &ComPtr<IForceFeedbackEffect>) -> Result<ComPtr<foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).LoadEffectAsync)(self.deref() as *const _ as *mut _, effect.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn pause_all_effects(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).PauseAllEffects)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn resume_all_effects(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ResumeAllEffects)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop_all_effects(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).StopAllEffects)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_disable_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).TryDisableAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_enable_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).TryEnableAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_reset_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).TryResetAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_unload_effect_async(&self, effect: &ComPtr<IForceFeedbackEffect>) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).TryUnloadEffectAsync)(self.deref() as *const _ as *mut _, effect.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ForceFeedbackMotor: IForceFeedbackMotor}
DEFINE_IID!(IID_IPeriodicForceEffect, 1548826839, 64629, 19794, 154, 10, 239, 228, 202, 181, 254, 100);
RT_INTERFACE!{interface IPeriodicForceEffect(IPeriodicForceEffectVtbl): IInspectable(IInspectableVtbl) [IID_IPeriodicForceEffect] {
    fn get_Kind(&self, out: *mut PeriodicForceEffectKind) -> HRESULT,
    fn SetParameters(&self, vector: foundation::numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: foundation::TimeSpan) -> HRESULT,
    fn SetParametersWithEnvelope(&self, vector: foundation::numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: foundation::TimeSpan, attackDuration: foundation::TimeSpan, sustainDuration: foundation::TimeSpan, releaseDuration: foundation::TimeSpan, repeatCount: u32) -> HRESULT
}}
impl ComPtr<IPeriodicForceEffect> {
    #[inline] pub fn get_kind(&self) -> Result<PeriodicForceEffectKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Kind)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_parameters(&self, vector: foundation::numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SetParameters)(self.deref() as *const _ as *mut _, vector, frequency, phase, bias, duration);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_parameters_with_envelope(&self, vector: foundation::numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: foundation::TimeSpan, attackDuration: foundation::TimeSpan, sustainDuration: foundation::TimeSpan, releaseDuration: foundation::TimeSpan, repeatCount: u32) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SetParametersWithEnvelope)(self.deref() as *const _ as *mut _, vector, frequency, phase, bias, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PeriodicForceEffect: IForceFeedbackEffect}
impl RtActivatable<IPeriodicForceEffectFactory> for PeriodicForceEffect {}
impl PeriodicForceEffect {
    #[inline] pub fn create_instance(effectKind: PeriodicForceEffectKind) -> Result<ComPtr<PeriodicForceEffect>> {
        <Self as RtActivatable<IPeriodicForceEffectFactory>>::get_activation_factory().create_instance(effectKind)
    }
}
DEFINE_CLSID!(PeriodicForceEffect(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,70,111,114,99,101,70,101,101,100,98,97,99,107,46,80,101,114,105,111,100,105,99,70,111,114,99,101,69,102,102,101,99,116,0]) [CLSID_PeriodicForceEffect]);
DEFINE_IID!(IID_IPeriodicForceEffectFactory, 1868753690, 38993, 18299, 179, 24, 53, 236, 170, 21, 7, 15);
RT_INTERFACE!{static interface IPeriodicForceEffectFactory(IPeriodicForceEffectFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPeriodicForceEffectFactory] {
    fn CreateInstance(&self, effectKind: PeriodicForceEffectKind, out: *mut *mut PeriodicForceEffect) -> HRESULT
}}
impl ComPtr<IPeriodicForceEffectFactory> {
    #[inline] pub fn create_instance(&self, effectKind: PeriodicForceEffectKind) -> Result<ComPtr<PeriodicForceEffect>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateInstance)(self.deref() as *const _ as *mut _, effectKind, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum PeriodicForceEffectKind: i32 {
    SquareWave = 0, SineWave = 1, TriangleWave = 2, SawtoothWaveUp = 3, SawtoothWaveDown = 4,
}}
DEFINE_IID!(IID_IRampForceEffect, 4059566681, 7334, 16512, 181, 109, 180, 63, 51, 84, 208, 82);
RT_INTERFACE!{interface IRampForceEffect(IRampForceEffectVtbl): IInspectable(IInspectableVtbl) [IID_IRampForceEffect] {
    fn SetParameters(&self, startVector: foundation::numerics::Vector3, endVector: foundation::numerics::Vector3, duration: foundation::TimeSpan) -> HRESULT,
    fn SetParametersWithEnvelope(&self, startVector: foundation::numerics::Vector3, endVector: foundation::numerics::Vector3, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: foundation::TimeSpan, attackDuration: foundation::TimeSpan, sustainDuration: foundation::TimeSpan, releaseDuration: foundation::TimeSpan, repeatCount: u32) -> HRESULT
}}
impl ComPtr<IRampForceEffect> {
    #[inline] pub fn set_parameters(&self, startVector: foundation::numerics::Vector3, endVector: foundation::numerics::Vector3, duration: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SetParameters)(self.deref() as *const _ as *mut _, startVector, endVector, duration);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_parameters_with_envelope(&self, startVector: foundation::numerics::Vector3, endVector: foundation::numerics::Vector3, attackGain: f32, sustainGain: f32, releaseGain: f32, startDelay: foundation::TimeSpan, attackDuration: foundation::TimeSpan, sustainDuration: foundation::TimeSpan, releaseDuration: foundation::TimeSpan, repeatCount: u32) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SetParametersWithEnvelope)(self.deref() as *const _ as *mut _, startVector, endVector, attackGain, sustainGain, releaseGain, startDelay, attackDuration, sustainDuration, releaseDuration, repeatCount);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class RampForceEffect: IForceFeedbackEffect}
impl RtActivatable<IActivationFactory> for RampForceEffect {}
DEFINE_CLSID!(RampForceEffect(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,70,111,114,99,101,70,101,101,100,98,97,99,107,46,82,97,109,112,70,111,114,99,101,69,102,102,101,99,116,0]) [CLSID_RampForceEffect]);
} // Windows.Gaming.Input.ForceFeedback
pub mod preview { // Windows.Gaming.Input.Preview
use crate::prelude::*;
RT_CLASS!{static class GameControllerProviderInfo}
impl RtActivatable<IGameControllerProviderInfoStatics> for GameControllerProviderInfo {}
impl GameControllerProviderInfo {
    #[inline] pub fn get_parent_provider_id(provider: &ComPtr<super::custom::IGameControllerProvider>) -> Result<HString> {
        <Self as RtActivatable<IGameControllerProviderInfoStatics>>::get_activation_factory().get_parent_provider_id(provider)
    }
    #[inline] pub fn get_provider_id(provider: &ComPtr<super::custom::IGameControllerProvider>) -> Result<HString> {
        <Self as RtActivatable<IGameControllerProviderInfoStatics>>::get_activation_factory().get_provider_id(provider)
    }
}
DEFINE_CLSID!(GameControllerProviderInfo(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,73,110,112,117,116,46,80,114,101,118,105,101,119,46,71,97,109,101,67,111,110,116,114,111,108,108,101,114,80,114,111,118,105,100,101,114,73,110,102,111,0]) [CLSID_GameControllerProviderInfo]);
DEFINE_IID!(IID_IGameControllerProviderInfoStatics, 199354053, 55741, 17646, 131, 98, 72, 139, 46, 70, 75, 251);
RT_INTERFACE!{static interface IGameControllerProviderInfoStatics(IGameControllerProviderInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameControllerProviderInfoStatics] {
    fn GetParentProviderId(&self, provider: *mut super::custom::IGameControllerProvider, out: *mut HSTRING) -> HRESULT,
    fn GetProviderId(&self, provider: *mut super::custom::IGameControllerProvider, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IGameControllerProviderInfoStatics> {
    #[inline] pub fn get_parent_provider_id(&self, provider: &ComPtr<super::custom::IGameControllerProvider>) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetParentProviderId)(self.deref() as *const _ as *mut _, provider.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_id(&self, provider: &ComPtr<super::custom::IGameControllerProvider>) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetProviderId)(self.deref() as *const _ as *mut _, provider.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.Gaming.Input.Preview
} // Windows.Gaming.Input
pub mod preview { // Windows.Gaming.Preview
pub mod gamesenumeration { // Windows.Gaming.Preview.GamesEnumeration
use crate::prelude::*;
RT_CLASS!{static class GameList}
impl RtActivatable<IGameListStatics> for GameList {}
impl RtActivatable<IGameListStatics2> for GameList {}
impl GameList {
    #[inline] pub fn find_all_async() -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<GameListEntry>>>> {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().find_all_async()
    }
    #[inline] pub fn find_all_async_package_family_name(packageFamilyName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<GameListEntry>>>> {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().find_all_async_package_family_name(packageFamilyName)
    }
    #[inline] pub fn add_game_added(handler: &ComPtr<GameListChangedEventHandler>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().add_game_added(handler)
    }
    #[inline] pub fn remove_game_added(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().remove_game_added(token)
    }
    #[inline] pub fn add_game_removed(handler: &ComPtr<GameListRemovedEventHandler>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().add_game_removed(handler)
    }
    #[inline] pub fn remove_game_removed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().remove_game_removed(token)
    }
    #[inline] pub fn add_game_updated(handler: &ComPtr<GameListChangedEventHandler>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().add_game_updated(handler)
    }
    #[inline] pub fn remove_game_updated(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IGameListStatics>>::get_activation_factory().remove_game_updated(token)
    }
    #[inline] pub fn merge_entries_async(left: &ComPtr<GameListEntry>, right: &ComPtr<GameListEntry>) -> Result<ComPtr<foundation::IAsyncOperation<GameListEntry>>> {
        <Self as RtActivatable<IGameListStatics2>>::get_activation_factory().merge_entries_async(left, right)
    }
    #[inline] pub fn unmerge_entry_async(mergedEntry: &ComPtr<GameListEntry>) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<GameListEntry>>>> {
        <Self as RtActivatable<IGameListStatics2>>::get_activation_factory().unmerge_entry_async(mergedEntry)
    }
}
DEFINE_CLSID!(GameList(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,80,114,101,118,105,101,119,46,71,97,109,101,115,69,110,117,109,101,114,97,116,105,111,110,46,71,97,109,101,76,105,115,116,0]) [CLSID_GameList]);
RT_ENUM! { enum GameListCategory: i32 {
    Candidate = 0, ConfirmedBySystem = 1, ConfirmedByUser = 2,
}}
DEFINE_IID!(IID_GameListChangedEventHandler, 636920865, 55541, 19857, 180, 14, 83, 213, 232, 111, 222, 100);
RT_DELEGATE!{delegate GameListChangedEventHandler(GameListChangedEventHandlerVtbl, GameListChangedEventHandlerImpl) [IID_GameListChangedEventHandler] {
    fn Invoke(&self, game: *mut GameListEntry) -> HRESULT
}}
impl ComPtr<GameListChangedEventHandler> {
    #[inline] pub fn invoke(&self, game: &ComPtr<GameListEntry>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Invoke)(self.deref() as *const _ as *mut _, game.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGameListEntry, 1935221971, 33055, 17556, 182, 156, 198, 65, 160, 198, 21, 67);
RT_INTERFACE!{interface IGameListEntry(IGameListEntryVtbl): IInspectable(IInspectableVtbl) [IID_IGameListEntry] {
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn get_DisplayInfo(&self, out: *mut *mut crate::windows::applicationmodel::AppDisplayInfo) -> HRESULT,
    fn LaunchAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn get_Category(&self, out: *mut GameListCategory) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut foundation::collections::IMapView<HString, IInspectable>) -> HRESULT,
    fn SetCategoryAsync(&self, value: GameListCategory, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl ComPtr<IGameListEntry> {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_display_info(&self) -> Result<Option<ComPtr<crate::windows::applicationmodel::AppDisplayInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_DisplayInfo)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).LaunchAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_category(&self) -> Result<GameListCategory> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Category)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, IInspectable>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Properties)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_category_async(&self, value: GameListCategory) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).SetCategoryAsync)(self.deref() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GameListEntry: IGameListEntry}
DEFINE_IID!(IID_IGameListEntry2, 3628765067, 34633, 18981, 144, 211, 246, 197, 164, 39, 136, 109);
RT_INTERFACE!{interface IGameListEntry2(IGameListEntry2Vtbl): IInspectable(IInspectableVtbl) [IID_IGameListEntry2] {
    fn get_LaunchableState(&self, out: *mut GameListEntryLaunchableState) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_LauncherExecutable(&self, out: *mut *mut crate::windows::storage::IStorageFile) -> HRESULT,
    fn get_LaunchParameters(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetLauncherExecutableFileAsync(&self, executableFile: *mut crate::windows::storage::IStorageFile, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetLauncherExecutableFileWithParamsAsync(&self, executableFile: *mut crate::windows::storage::IStorageFile, launchParams: HSTRING, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn get_TitleId(&self, out: *mut HSTRING) -> HRESULT,
    fn SetTitleIdAsync(&self, id: HSTRING, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn get_GameModeConfiguration(&self, out: *mut *mut GameModeConfiguration) -> HRESULT
}}
impl ComPtr<IGameListEntry2> {
    #[inline] pub fn get_launchable_state(&self) -> Result<GameListEntryLaunchableState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_LaunchableState)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_launcher_executable(&self) -> Result<Option<ComPtr<crate::windows::storage::IStorageFile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_LauncherExecutable)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_launch_parameters(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_LaunchParameters)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_launcher_executable_file_async(&self, executableFile: &ComPtr<crate::windows::storage::IStorageFile>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).SetLauncherExecutableFileAsync)(self.deref() as *const _ as *mut _, executableFile.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_launcher_executable_file_with_params_async(&self, executableFile: &ComPtr<crate::windows::storage::IStorageFile>, launchParams: &HStringArg) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).SetLauncherExecutableFileWithParamsAsync)(self.deref() as *const _ as *mut _, executableFile.deref() as *const _ as *mut _, launchParams.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_title_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_TitleId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_title_id_async(&self, id: &HStringArg) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).SetTitleIdAsync)(self.deref() as *const _ as *mut _, id.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_game_mode_configuration(&self) -> Result<Option<ComPtr<GameModeConfiguration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_GameModeConfiguration)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum GameListEntryLaunchableState: i32 {
    NotLaunchable = 0, ByLastRunningFullPath = 1, ByUserProvidedPath = 2, ByTile = 3,
}}
DEFINE_IID!(IID_GameListRemovedEventHandler, 281371791, 27791, 18194, 155, 56, 71, 75, 194, 46, 118, 216);
RT_DELEGATE!{delegate GameListRemovedEventHandler(GameListRemovedEventHandlerVtbl, GameListRemovedEventHandlerImpl) [IID_GameListRemovedEventHandler] {
    fn Invoke(&self, identifier: HSTRING) -> HRESULT
}}
impl ComPtr<GameListRemovedEventHandler> {
    #[inline] pub fn invoke(&self, identifier: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Invoke)(self.deref() as *const _ as *mut _, identifier.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGameListStatics, 769462127, 40038, 19205, 148, 92, 214, 237, 120, 73, 27, 140);
RT_INTERFACE!{static interface IGameListStatics(IGameListStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameListStatics] {
    fn FindAllAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<GameListEntry>>) -> HRESULT,
    fn FindAllAsyncPackageFamilyName(&self, packageFamilyName: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<GameListEntry>>) -> HRESULT,
    fn add_GameAdded(&self, handler: *mut GameListChangedEventHandler, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GameAdded(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_GameRemoved(&self, handler: *mut GameListRemovedEventHandler, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GameRemoved(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_GameUpdated(&self, handler: *mut GameListChangedEventHandler, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GameUpdated(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl ComPtr<IGameListStatics> {
    #[inline] pub fn find_all_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<GameListEntry>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).FindAllAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_all_async_package_family_name(&self, packageFamilyName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<GameListEntry>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).FindAllAsyncPackageFamilyName)(self.deref() as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_game_added(&self, handler: &ComPtr<GameListChangedEventHandler>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_GameAdded)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_game_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_GameAdded)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_game_removed(&self, handler: &ComPtr<GameListRemovedEventHandler>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_GameRemoved)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_game_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_GameRemoved)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_game_updated(&self, handler: &ComPtr<GameListChangedEventHandler>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_GameUpdated)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_game_updated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_GameUpdated)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGameListStatics2, 962535576, 59930, 17834, 146, 104, 168, 57, 5, 104, 111, 39);
RT_INTERFACE!{static interface IGameListStatics2(IGameListStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IGameListStatics2] {
    fn MergeEntriesAsync(&self, left: *mut GameListEntry, right: *mut GameListEntry, out: *mut *mut foundation::IAsyncOperation<GameListEntry>) -> HRESULT,
    fn UnmergeEntryAsync(&self, mergedEntry: *mut GameListEntry, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<GameListEntry>>) -> HRESULT
}}
impl ComPtr<IGameListStatics2> {
    #[inline] pub fn merge_entries_async(&self, left: &ComPtr<GameListEntry>, right: &ComPtr<GameListEntry>) -> Result<ComPtr<foundation::IAsyncOperation<GameListEntry>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).MergeEntriesAsync)(self.deref() as *const _ as *mut _, left.deref() as *const _ as *mut _, right.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn unmerge_entry_async(&self, mergedEntry: &ComPtr<GameListEntry>) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<GameListEntry>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).UnmergeEntryAsync)(self.deref() as *const _ as *mut _, mergedEntry.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGameModeConfiguration, 2028310959, 45378, 20208, 136, 48, 85, 188, 43, 228, 245, 234);
RT_INTERFACE!{interface IGameModeConfiguration(IGameModeConfigurationVtbl): IInspectable(IInspectableVtbl) [IID_IGameModeConfiguration] {
    fn get_IsEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsEnabled(&self, value: bool) -> HRESULT,
    fn get_RelatedProcessNames(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT,
    fn get_PercentGpuTimeAllocatedToGame(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn put_PercentGpuTimeAllocatedToGame(&self, value: *mut foundation::IReference<i32>) -> HRESULT,
    fn get_PercentGpuMemoryAllocatedToGame(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn put_PercentGpuMemoryAllocatedToGame(&self, value: *mut foundation::IReference<i32>) -> HRESULT,
    fn get_PercentGpuMemoryAllocatedToSystemCompositor(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn put_PercentGpuMemoryAllocatedToSystemCompositor(&self, value: *mut foundation::IReference<i32>) -> HRESULT,
    fn get_MaxCpuCount(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn put_MaxCpuCount(&self, value: *mut foundation::IReference<i32>) -> HRESULT,
    fn get_CpuExclusivityMaskLow(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn put_CpuExclusivityMaskLow(&self, value: *mut foundation::IReference<i32>) -> HRESULT,
    fn get_CpuExclusivityMaskHigh(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn put_CpuExclusivityMaskHigh(&self, value: *mut foundation::IReference<i32>) -> HRESULT,
    fn get_AffinitizeToExclusiveCpus(&self, out: *mut bool) -> HRESULT,
    fn put_AffinitizeToExclusiveCpus(&self, value: bool) -> HRESULT,
    fn SaveAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl ComPtr<IGameModeConfiguration> {
    #[inline] pub fn get_is_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsEnabled)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_IsEnabled)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_related_process_names(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_RelatedProcessNames)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_percent_gpu_time_allocated_to_game(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_PercentGpuTimeAllocatedToGame)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_percent_gpu_time_allocated_to_game(&self, value: &ComPtr<foundation::IReference<i32>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_PercentGpuTimeAllocatedToGame)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_percent_gpu_memory_allocated_to_game(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_PercentGpuMemoryAllocatedToGame)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_percent_gpu_memory_allocated_to_game(&self, value: &ComPtr<foundation::IReference<i32>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_PercentGpuMemoryAllocatedToGame)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_percent_gpu_memory_allocated_to_system_compositor(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_PercentGpuMemoryAllocatedToSystemCompositor)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_percent_gpu_memory_allocated_to_system_compositor(&self, value: &ComPtr<foundation::IReference<i32>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_PercentGpuMemoryAllocatedToSystemCompositor)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_cpu_count(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_MaxCpuCount)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_max_cpu_count(&self, value: &ComPtr<foundation::IReference<i32>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MaxCpuCount)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_cpu_exclusivity_mask_low(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_CpuExclusivityMaskLow)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_cpu_exclusivity_mask_low(&self, value: &ComPtr<foundation::IReference<i32>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_CpuExclusivityMaskLow)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_cpu_exclusivity_mask_high(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_CpuExclusivityMaskHigh)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_cpu_exclusivity_mask_high(&self, value: &ComPtr<foundation::IReference<i32>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_CpuExclusivityMaskHigh)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_affinitize_to_exclusive_cpus(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_AffinitizeToExclusiveCpus)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_affinitize_to_exclusive_cpus(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_AffinitizeToExclusiveCpus)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn save_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).SaveAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GameModeConfiguration: IGameModeConfiguration}
DEFINE_IID!(IID_IGameModeUserConfiguration, 1926449908, 30059, 18191, 160, 194, 186, 98, 169, 7, 149, 219);
RT_INTERFACE!{interface IGameModeUserConfiguration(IGameModeUserConfigurationVtbl): IInspectable(IInspectableVtbl) [IID_IGameModeUserConfiguration] {
    fn get_GamingRelatedProcessNames(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT,
    fn SaveAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl ComPtr<IGameModeUserConfiguration> {
    #[inline] pub fn get_gaming_related_process_names(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_GamingRelatedProcessNames)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn save_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).SaveAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GameModeUserConfiguration: IGameModeUserConfiguration}
impl RtActivatable<IGameModeUserConfigurationStatics> for GameModeUserConfiguration {}
impl GameModeUserConfiguration {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<GameModeUserConfiguration>>> {
        <Self as RtActivatable<IGameModeUserConfigurationStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(GameModeUserConfiguration(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,80,114,101,118,105,101,119,46,71,97,109,101,115,69,110,117,109,101,114,97,116,105,111,110,46,71,97,109,101,77,111,100,101,85,115,101,114,67,111,110,102,105,103,117,114,97,116,105,111,110,0]) [CLSID_GameModeUserConfiguration]);
DEFINE_IID!(IID_IGameModeUserConfigurationStatics, 1850792316, 26346, 18318, 164, 161, 245, 124, 14, 141, 0, 231);
RT_INTERFACE!{static interface IGameModeUserConfigurationStatics(IGameModeUserConfigurationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameModeUserConfigurationStatics] {
    fn GetDefault(&self, out: *mut *mut GameModeUserConfiguration) -> HRESULT
}}
impl ComPtr<IGameModeUserConfigurationStatics> {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<GameModeUserConfiguration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetDefault)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
} // Windows.Gaming.Preview.GamesEnumeration
} // Windows.Gaming.Preview
pub mod ui { // Windows.Gaming.UI
use crate::prelude::*;
RT_CLASS!{static class GameBar}
impl RtActivatable<IGameBarStatics> for GameBar {}
impl GameBar {
    #[inline] pub fn add_visibility_changed(handler: &ComPtr<foundation::EventHandler<IInspectable>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().add_visibility_changed(handler)
    }
    #[inline] pub fn remove_visibility_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().remove_visibility_changed(token)
    }
    #[inline] pub fn add_is_input_redirected_changed(handler: &ComPtr<foundation::EventHandler<IInspectable>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().add_is_input_redirected_changed(handler)
    }
    #[inline] pub fn remove_is_input_redirected_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().remove_is_input_redirected_changed(token)
    }
    #[inline] pub fn get_visible() -> Result<bool> {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().get_visible()
    }
    #[inline] pub fn get_is_input_redirected() -> Result<bool> {
        <Self as RtActivatable<IGameBarStatics>>::get_activation_factory().get_is_input_redirected()
    }
}
DEFINE_CLSID!(GameBar(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,85,73,46,71,97,109,101,66,97,114,0]) [CLSID_GameBar]);
DEFINE_IID!(IID_IGameBarStatics, 498705042, 52344, 16755, 190, 69, 182, 30, 103, 40, 62, 167);
RT_INTERFACE!{static interface IGameBarStatics(IGameBarStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameBarStatics] {
    fn add_VisibilityChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_VisibilityChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_IsInputRedirectedChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_IsInputRedirectedChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Visible(&self, out: *mut bool) -> HRESULT,
    fn get_IsInputRedirected(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IGameBarStatics> {
    #[inline] pub fn add_visibility_changed(&self, handler: &ComPtr<foundation::EventHandler<IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_VisibilityChanged)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_visibility_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_VisibilityChanged)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_is_input_redirected_changed(&self, handler: &ComPtr<foundation::EventHandler<IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_IsInputRedirectedChanged)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_is_input_redirected_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_IsInputRedirectedChanged)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_visible(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Visible)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_input_redirected(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsInputRedirected)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum GameChatMessageOrigin: i32 {
    Voice = 0, Text = 1,
}}
DEFINE_IID!(IID_IGameChatMessageReceivedEventArgs, 2726429169, 16313, 20034, 164, 3, 122, 252, 226, 2, 59, 30);
RT_INTERFACE!{interface IGameChatMessageReceivedEventArgs(IGameChatMessageReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IGameChatMessageReceivedEventArgs] {
    fn get_AppId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AppDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SenderName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Message(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Origin(&self, out: *mut GameChatMessageOrigin) -> HRESULT
}}
impl ComPtr<IGameChatMessageReceivedEventArgs> {
    #[inline] pub fn get_app_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_AppId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_app_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_AppDisplayName)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sender_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_SenderName)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_message(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Message)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_origin(&self) -> Result<GameChatMessageOrigin> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Origin)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class GameChatMessageReceivedEventArgs: IGameChatMessageReceivedEventArgs}
DEFINE_IID!(IID_IGameChatOverlay, 4224075877, 63228, 19016, 174, 7, 3, 172, 110, 212, 55, 4);
RT_INTERFACE!{interface IGameChatOverlay(IGameChatOverlayVtbl): IInspectable(IInspectableVtbl) [IID_IGameChatOverlay] {
    fn get_DesiredPosition(&self, out: *mut GameChatOverlayPosition) -> HRESULT,
    fn put_DesiredPosition(&self, value: GameChatOverlayPosition) -> HRESULT,
    fn AddMessage(&self, sender: HSTRING, message: HSTRING, origin: GameChatMessageOrigin) -> HRESULT
}}
impl ComPtr<IGameChatOverlay> {
    #[inline] pub fn get_desired_position(&self) -> Result<GameChatOverlayPosition> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_DesiredPosition)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_desired_position(&self, value: GameChatOverlayPosition) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_DesiredPosition)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_message(&self, sender: &HStringArg, message: &HStringArg, origin: GameChatMessageOrigin) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).AddMessage)(self.deref() as *const _ as *mut _, sender.get(), message.get(), origin);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class GameChatOverlay: IGameChatOverlay}
impl RtActivatable<IGameChatOverlayStatics> for GameChatOverlay {}
impl GameChatOverlay {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<GameChatOverlay>>> {
        <Self as RtActivatable<IGameChatOverlayStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(GameChatOverlay(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,85,73,46,71,97,109,101,67,104,97,116,79,118,101,114,108,97,121,0]) [CLSID_GameChatOverlay]);
DEFINE_IID!(IID_IGameChatOverlayMessageSource, 504853399, 23035, 20303, 142, 154, 128, 172, 248, 23, 116, 60);
RT_INTERFACE!{interface IGameChatOverlayMessageSource(IGameChatOverlayMessageSourceVtbl): IInspectable(IInspectableVtbl) [IID_IGameChatOverlayMessageSource] {
    fn add_MessageReceived(&self, handler: *mut foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_MessageReceived(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn SetDelayBeforeClosingAfterMessageReceived(&self, value: foundation::TimeSpan) -> HRESULT
}}
impl ComPtr<IGameChatOverlayMessageSource> {
    #[inline] pub fn add_message_received(&self, handler: &ComPtr<foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_MessageReceived)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_message_received(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_MessageReceived)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_delay_before_closing_after_message_received(&self, value: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SetDelayBeforeClosingAfterMessageReceived)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class GameChatOverlayMessageSource: IGameChatOverlayMessageSource}
impl RtActivatable<IActivationFactory> for GameChatOverlayMessageSource {}
DEFINE_CLSID!(GameChatOverlayMessageSource(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,85,73,46,71,97,109,101,67,104,97,116,79,118,101,114,108,97,121,77,101,115,115,97,103,101,83,111,117,114,99,101,0]) [CLSID_GameChatOverlayMessageSource]);
RT_ENUM! { enum GameChatOverlayPosition: i32 {
    BottomCenter = 0, BottomLeft = 1, BottomRight = 2, MiddleRight = 3, MiddleLeft = 4, TopCenter = 5, TopLeft = 6, TopRight = 7,
}}
DEFINE_IID!(IID_IGameChatOverlayStatics, 2309813780, 30823, 18935, 150, 135, 37, 217, 219, 244, 68, 209);
RT_INTERFACE!{static interface IGameChatOverlayStatics(IGameChatOverlayStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameChatOverlayStatics] {
    fn GetDefault(&self, out: *mut *mut GameChatOverlay) -> HRESULT
}}
impl ComPtr<IGameChatOverlayStatics> {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<GameChatOverlay>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetDefault)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGameUIProviderActivatedEventArgs, 2813534270, 51959, 19949, 187, 210, 71, 222, 67, 187, 109, 213);
RT_INTERFACE!{interface IGameUIProviderActivatedEventArgs(IGameUIProviderActivatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IGameUIProviderActivatedEventArgs] {
    fn get_GameUIArgs(&self, out: *mut *mut foundation::collections::ValueSet) -> HRESULT,
    fn ReportCompleted(&self, results: *mut foundation::collections::ValueSet) -> HRESULT
}}
impl ComPtr<IGameUIProviderActivatedEventArgs> {
    #[inline] pub fn get_game_ui_args(&self) -> Result<Option<ComPtr<foundation::collections::ValueSet>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_GameUIArgs)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn report_completed(&self, results: &ComPtr<foundation::collections::ValueSet>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ReportCompleted)(self.deref() as *const _ as *mut _, results.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class GameUIProviderActivatedEventArgs: IGameUIProviderActivatedEventArgs}
} // Windows.Gaming.UI
pub mod xboxlive { // Windows.Gaming.XboxLive
pub mod storage { // Windows.Gaming.XboxLive.Storage
use crate::prelude::*;
DEFINE_IID!(IID_IGameSaveBlobGetResult, 2440200672, 29185, 18771, 170, 44, 64, 8, 240, 58, 239, 69);
RT_INTERFACE!{interface IGameSaveBlobGetResult(IGameSaveBlobGetResultVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveBlobGetResult] {
    fn get_Status(&self, out: *mut GameSaveErrorStatus) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_Value(&self, out: *mut *mut foundation::collections::IMapView<HString, crate::windows::storage::streams::IBuffer>) -> HRESULT
}}
impl ComPtr<IGameSaveBlobGetResult> {
    #[inline] pub fn get_status(&self) -> Result<GameSaveErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Status)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_value(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, crate::windows::storage::streams::IBuffer>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Value)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GameSaveBlobGetResult: IGameSaveBlobGetResult}
DEFINE_IID!(IID_IGameSaveBlobInfo, 2916319284, 47856, 17989, 182, 208, 70, 237, 175, 251, 60, 43);
RT_INTERFACE!{interface IGameSaveBlobInfo(IGameSaveBlobInfoVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveBlobInfo] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Size(&self, out: *mut u32) -> HRESULT
}}
impl ComPtr<IGameSaveBlobInfo> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Name)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_size(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Size)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class GameSaveBlobInfo: IGameSaveBlobInfo}
DEFINE_IID!(IID_IGameSaveBlobInfoGetResult, 3344401794, 13975, 17087, 152, 156, 102, 93, 146, 59, 82, 49);
RT_INTERFACE!{interface IGameSaveBlobInfoGetResult(IGameSaveBlobInfoGetResultVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveBlobInfoGetResult] {
    fn get_Status(&self, out: *mut GameSaveErrorStatus) -> HRESULT,
    fn get_Value(&self, out: *mut *mut foundation::collections::IVectorView<GameSaveBlobInfo>) -> HRESULT
}}
impl ComPtr<IGameSaveBlobInfoGetResult> {
    #[inline] pub fn get_status(&self) -> Result<GameSaveErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Status)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<GameSaveBlobInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Value)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GameSaveBlobInfoGetResult: IGameSaveBlobInfoGetResult}
DEFINE_IID!(IID_IGameSaveBlobInfoQuery, 2682090674, 61166, 17531, 169, 210, 127, 150, 192, 248, 50, 8);
RT_INTERFACE!{interface IGameSaveBlobInfoQuery(IGameSaveBlobInfoQueryVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveBlobInfoQuery] {
    fn GetBlobInfoAsync(&self, out: *mut *mut foundation::IAsyncOperation<GameSaveBlobInfoGetResult>) -> HRESULT,
    fn GetBlobInfoWithIndexAndMaxAsync(&self, startIndex: u32, maxNumberOfItems: u32, out: *mut *mut foundation::IAsyncOperation<GameSaveBlobInfoGetResult>) -> HRESULT,
    fn GetItemCountAsync(&self, out: *mut *mut foundation::IAsyncOperation<u32>) -> HRESULT
}}
impl ComPtr<IGameSaveBlobInfoQuery> {
    #[inline] pub fn get_blob_info_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetBlobInfoAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_blob_info_with_index_and_max_async(&self, startIndex: u32, maxNumberOfItems: u32) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveBlobInfoGetResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetBlobInfoWithIndexAndMaxAsync)(self.deref() as *const _ as *mut _, startIndex, maxNumberOfItems, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_item_count_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<u32>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetItemCountAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GameSaveBlobInfoQuery: IGameSaveBlobInfoQuery}
DEFINE_IID!(IID_IGameSaveContainer, 3284176777, 22079, 20173, 156, 111, 51, 253, 14, 50, 61, 16);
RT_INTERFACE!{interface IGameSaveContainer(IGameSaveContainerVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveContainer] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Provider(&self, out: *mut *mut GameSaveProvider) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn SubmitUpdatesAsync(&self, blobsToWrite: *mut foundation::collections::IMapView<HString, crate::windows::storage::streams::IBuffer>, blobsToDelete: *mut foundation::collections::IIterable<HString>, displayName: HSTRING, out: *mut *mut foundation::IAsyncOperation<GameSaveOperationResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn ReadAsync(&self, blobsToRead: *mut foundation::collections::IMapView<HString, crate::windows::storage::streams::IBuffer>, out: *mut *mut foundation::IAsyncOperation<GameSaveOperationResult>) -> HRESULT,
    fn GetAsync(&self, blobsToRead: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperation<GameSaveBlobGetResult>) -> HRESULT,
    fn SubmitPropertySetUpdatesAsync(&self, blobsToWrite: *mut foundation::collections::IPropertySet, blobsToDelete: *mut foundation::collections::IIterable<HString>, displayName: HSTRING, out: *mut *mut foundation::IAsyncOperation<GameSaveOperationResult>) -> HRESULT,
    fn CreateBlobInfoQuery(&self, blobNamePrefix: HSTRING, out: *mut *mut GameSaveBlobInfoQuery) -> HRESULT
}}
impl ComPtr<IGameSaveContainer> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Name)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider(&self) -> Result<Option<ComPtr<GameSaveProvider>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Provider)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn submit_updates_async(&self, blobsToWrite: &ComPtr<foundation::collections::IMapView<HString, crate::windows::storage::streams::IBuffer>>, blobsToDelete: &ComPtr<foundation::collections::IIterable<HString>>, displayName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).SubmitUpdatesAsync)(self.deref() as *const _ as *mut _, blobsToWrite.deref() as *const _ as *mut _, blobsToDelete.deref() as *const _ as *mut _, displayName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn read_async(&self, blobsToRead: &ComPtr<foundation::collections::IMapView<HString, crate::windows::storage::streams::IBuffer>>) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).ReadAsync)(self.deref() as *const _ as *mut _, blobsToRead.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_async(&self, blobsToRead: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveBlobGetResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetAsync)(self.deref() as *const _ as *mut _, blobsToRead.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn submit_property_set_updates_async(&self, blobsToWrite: &ComPtr<foundation::collections::IPropertySet>, blobsToDelete: &ComPtr<foundation::collections::IIterable<HString>>, displayName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).SubmitPropertySetUpdatesAsync)(self.deref() as *const _ as *mut _, blobsToWrite.deref() as *const _ as *mut _, blobsToDelete.deref() as *const _ as *mut _, displayName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_blob_info_query(&self, blobNamePrefix: &HStringArg) -> Result<Option<ComPtr<GameSaveBlobInfoQuery>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateBlobInfoQuery)(self.deref() as *const _ as *mut _, blobNamePrefix.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GameSaveContainer: IGameSaveContainer}
DEFINE_IID!(IID_IGameSaveContainerInfo, 3085071104, 5469, 19380, 178, 186, 147, 3, 6, 243, 145, 181);
RT_INTERFACE!{interface IGameSaveContainerInfo(IGameSaveContainerInfoVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveContainerInfo] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TotalSize(&self, out: *mut u64) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LastModifiedTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_NeedsSync(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IGameSaveContainerInfo> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Name)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_total_size(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_TotalSize)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_DisplayName)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_last_modified_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_LastModifiedTime)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_needs_sync(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_NeedsSync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class GameSaveContainerInfo: IGameSaveContainerInfo}
DEFINE_IID!(IID_IGameSaveContainerInfoGetResult, 4291104116, 50561, 20381, 158, 57, 48, 161, 12, 30, 76, 80);
RT_INTERFACE!{interface IGameSaveContainerInfoGetResult(IGameSaveContainerInfoGetResultVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveContainerInfoGetResult] {
    fn get_Status(&self, out: *mut GameSaveErrorStatus) -> HRESULT,
    fn get_Value(&self, out: *mut *mut foundation::collections::IVectorView<GameSaveContainerInfo>) -> HRESULT
}}
impl ComPtr<IGameSaveContainerInfoGetResult> {
    #[inline] pub fn get_status(&self) -> Result<GameSaveErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Status)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<GameSaveContainerInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Value)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GameSaveContainerInfoGetResult: IGameSaveContainerInfoGetResult}
DEFINE_IID!(IID_IGameSaveContainerInfoQuery, 1016391779, 28544, 17191, 147, 39, 255, 193, 26, 253, 66, 179);
RT_INTERFACE!{interface IGameSaveContainerInfoQuery(IGameSaveContainerInfoQueryVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveContainerInfoQuery] {
    fn GetContainerInfoAsync(&self, out: *mut *mut foundation::IAsyncOperation<GameSaveContainerInfoGetResult>) -> HRESULT,
    fn GetContainerInfoWithIndexAndMaxAsync(&self, startIndex: u32, maxNumberOfItems: u32, out: *mut *mut foundation::IAsyncOperation<GameSaveContainerInfoGetResult>) -> HRESULT,
    fn GetItemCountAsync(&self, out: *mut *mut foundation::IAsyncOperation<u32>) -> HRESULT
}}
impl ComPtr<IGameSaveContainerInfoQuery> {
    #[inline] pub fn get_container_info_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetContainerInfoAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_container_info_with_index_and_max_async(&self, startIndex: u32, maxNumberOfItems: u32) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveContainerInfoGetResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetContainerInfoWithIndexAndMaxAsync)(self.deref() as *const _ as *mut _, startIndex, maxNumberOfItems, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_item_count_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<u32>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetItemCountAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GameSaveContainerInfoQuery: IGameSaveContainerInfoQuery}
RT_ENUM! { enum GameSaveErrorStatus: i32 {
    Ok = 0, Abort = -2147467260, InvalidContainerName = -2138898431, NoAccess = -2138898430, OutOfLocalStorage = -2138898429, UserCanceled = -2138898428, UpdateTooBig = -2138898427, QuotaExceeded = -2138898426, ProvidedBufferTooSmall = -2138898425, BlobNotFound = -2138898424, NoXboxLiveInfo = -2138898423, ContainerNotInSync = -2138898422, ContainerSyncFailed = -2138898421, UserHasNoXboxLiveInfo = -2138898420, ObjectExpired = -2138898419,
}}
DEFINE_IID!(IID_IGameSaveOperationResult, 3473873413, 9376, 17794, 154, 85, 177, 187, 187, 147, 136, 216);
RT_INTERFACE!{interface IGameSaveOperationResult(IGameSaveOperationResultVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveOperationResult] {
    fn get_Status(&self, out: *mut GameSaveErrorStatus) -> HRESULT
}}
impl ComPtr<IGameSaveOperationResult> {
    #[inline] pub fn get_status(&self) -> Result<GameSaveErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Status)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class GameSaveOperationResult: IGameSaveOperationResult}
DEFINE_IID!(IID_IGameSaveProvider, 2426798996, 33022, 16913, 151, 248, 165, 222, 20, 221, 149, 210);
RT_INTERFACE!{interface IGameSaveProvider(IGameSaveProviderVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveProvider] {
    #[cfg(not(feature="windows-system"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut *mut crate::windows::system::User) -> HRESULT,
    fn CreateContainer(&self, name: HSTRING, out: *mut *mut GameSaveContainer) -> HRESULT,
    fn DeleteContainerAsync(&self, name: HSTRING, out: *mut *mut foundation::IAsyncOperation<GameSaveOperationResult>) -> HRESULT,
    fn CreateContainerInfoQuery(&self, out: *mut *mut GameSaveContainerInfoQuery) -> HRESULT,
    fn CreateContainerInfoQueryWithName(&self, containerNamePrefix: HSTRING, out: *mut *mut GameSaveContainerInfoQuery) -> HRESULT,
    fn GetRemainingBytesInQuotaAsync(&self, out: *mut *mut foundation::IAsyncOperation<i64>) -> HRESULT,
    fn get_ContainersChangedSinceLastSync(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl ComPtr<IGameSaveProvider> {
    #[cfg(feature="windows-system")] #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<crate::windows::system::User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_User)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_container(&self, name: &HStringArg) -> Result<Option<ComPtr<GameSaveContainer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateContainer)(self.deref() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_container_async(&self, name: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).DeleteContainerAsync)(self.deref() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_container_info_query(&self) -> Result<Option<ComPtr<GameSaveContainerInfoQuery>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateContainerInfoQuery)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_container_info_query_with_name(&self, containerNamePrefix: &HStringArg) -> Result<Option<ComPtr<GameSaveContainerInfoQuery>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateContainerInfoQueryWithName)(self.deref() as *const _ as *mut _, containerNamePrefix.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remaining_bytes_in_quota_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<i64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetRemainingBytesInQuotaAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_containers_changed_since_last_sync(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ContainersChangedSinceLastSync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GameSaveProvider: IGameSaveProvider}
impl RtActivatable<IGameSaveProviderStatics> for GameSaveProvider {}
impl GameSaveProvider {
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user_async(user: &ComPtr<crate::windows::system::User>, serviceConfigId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveProviderGetResult>>> {
        <Self as RtActivatable<IGameSaveProviderStatics>>::get_activation_factory().get_for_user_async(user, serviceConfigId)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn get_sync_on_demand_for_user_async(user: &ComPtr<crate::windows::system::User>, serviceConfigId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveProviderGetResult>>> {
        <Self as RtActivatable<IGameSaveProviderStatics>>::get_activation_factory().get_sync_on_demand_for_user_async(user, serviceConfigId)
    }
}
DEFINE_CLSID!(GameSaveProvider(&[87,105,110,100,111,119,115,46,71,97,109,105,110,103,46,88,98,111,120,76,105,118,101,46,83,116,111,114,97,103,101,46,71,97,109,101,83,97,118,101,80,114,111,118,105,100,101,114,0]) [CLSID_GameSaveProvider]);
DEFINE_IID!(IID_IGameSaveProviderGetResult, 985204758, 54163, 19813, 172, 22, 65, 195, 230, 122, 185, 69);
RT_INTERFACE!{interface IGameSaveProviderGetResult(IGameSaveProviderGetResultVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveProviderGetResult] {
    fn get_Status(&self, out: *mut GameSaveErrorStatus) -> HRESULT,
    fn get_Value(&self, out: *mut *mut GameSaveProvider) -> HRESULT
}}
impl ComPtr<IGameSaveProviderGetResult> {
    #[inline] pub fn get_status(&self) -> Result<GameSaveErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Status)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<Option<ComPtr<GameSaveProvider>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Value)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GameSaveProviderGetResult: IGameSaveProviderGetResult}
DEFINE_IID!(IID_IGameSaveProviderStatics, 3491577552, 31491, 17565, 140, 189, 52, 2, 132, 42, 16, 72);
RT_INTERFACE!{static interface IGameSaveProviderStatics(IGameSaveProviderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGameSaveProviderStatics] {
    #[cfg(feature="windows-system")] fn GetForUserAsync(&self, user: *mut crate::windows::system::User, serviceConfigId: HSTRING, out: *mut *mut foundation::IAsyncOperation<GameSaveProviderGetResult>) -> HRESULT,
    #[cfg(feature="windows-system")] fn GetSyncOnDemandForUserAsync(&self, user: *mut crate::windows::system::User, serviceConfigId: HSTRING, out: *mut *mut foundation::IAsyncOperation<GameSaveProviderGetResult>) -> HRESULT
}}
impl ComPtr<IGameSaveProviderStatics> {
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user_async(&self, user: &ComPtr<crate::windows::system::User>, serviceConfigId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveProviderGetResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetForUserAsync)(self.deref() as *const _ as *mut _, user.deref() as *const _ as *mut _, serviceConfigId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_sync_on_demand_for_user_async(&self, user: &ComPtr<crate::windows::system::User>, serviceConfigId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<GameSaveProviderGetResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetSyncOnDemandForUserAsync)(self.deref() as *const _ as *mut _, user.deref() as *const _ as *mut _, serviceConfigId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.Gaming.XboxLive.Storage
} // Windows.Gaming.XboxLive
