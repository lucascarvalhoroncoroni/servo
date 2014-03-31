/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::BindingDeclarations::UIEventBinding;
use dom::bindings::codegen::InheritTypes::UIEventDerived;
use dom::bindings::js::{JS, JSRef, RootCollection, RootedReference, Unrooted};
use dom::bindings::error::Fallible;
use dom::bindings::utils::{Reflectable, Reflector, reflect_dom_object};
use dom::event::{Event, EventTypeId, UIEventTypeId};
use dom::node::Node;
use dom::window::Window;
use servo_util::str::DOMString;

use serialize::{Encoder, Encodable};

#[deriving(Encodable)]
pub struct UIEvent {
    pub event: Event,
    pub view: Option<JS<Window>>,
    pub detail: i32
}

impl UIEventDerived for Event {
    fn is_uievent(&self) -> bool {
        self.type_id == UIEventTypeId
    }
}

impl UIEvent {
    pub fn new_inherited(type_id: EventTypeId) -> UIEvent {
        UIEvent {
            event: Event::new_inherited(type_id),
            view: None,
            detail: 0
        }
    }

    pub fn new(window: &JSRef<Window>) -> Unrooted<UIEvent> {
        reflect_dom_object(~UIEvent::new_inherited(UIEventTypeId),
                           window,
                           UIEventBinding::Wrap)
    }

    pub fn Constructor(owner: &JSRef<Window>,
                       type_: DOMString,
                       init: &UIEventBinding::UIEventInit) -> Fallible<Unrooted<UIEvent>> {
        let roots = RootCollection::new();
        let mut ev = UIEvent::new(owner).root(&roots);
        let view = init.view.as_ref().map(|view| view.root(&roots));
        ev.get_mut().InitUIEvent(type_, init.parent.bubbles, init.parent.cancelable,
                                 view.root_ref(), init.detail);
        Ok(Unrooted::new_rooted(&*ev))
    }

    pub fn GetView(&self) -> Option<Unrooted<Window>> {
        self.view.clone().map(|view| Unrooted::new(view))
    }

    pub fn Detail(&self) -> i32 {
        self.detail
    }

    pub fn InitUIEvent(&mut self,
                       type_: DOMString,
                       can_bubble: bool,
                       cancelable: bool,
                       view: Option<JSRef<Window>>,
                       detail: i32) {
        self.event.InitEvent(type_, can_bubble, cancelable);
        self.view = view.map(|view| view.unrooted());
        self.detail = detail;
    }

    pub fn LayerX(&self) -> i32 {
        //TODO
        0
    }

    pub fn LayerY(&self) -> i32 {
        //TODO
        0
    }

    pub fn PageX(&self) -> i32 {
        //TODO
        0
    }

    pub fn PageY(&self) -> i32 {
        //TODO
        0
    }

    pub fn Which(&self) -> u32 {
        //TODO
        0
    }

    pub fn GetRangeParent(&self) -> Option<Unrooted<Node>> {
        //TODO
        None
    }

    pub fn RangeOffset(&self) -> i32 {
        //TODO
        0
    }

    pub fn CancelBubble(&self) -> bool {
        //TODO
        false
    }

    pub fn SetCancelBubble(&mut self, _val: bool) {
        //TODO
    }

    pub fn IsChar(&self) -> bool {
        //TODO
        false
    }
}

impl Reflectable for UIEvent {
    fn reflector<'a>(&'a self) -> &'a Reflector {
        self.event.reflector()
    }

    fn mut_reflector<'a>(&'a mut self) -> &'a mut Reflector {
        self.event.mut_reflector()
    }
}
