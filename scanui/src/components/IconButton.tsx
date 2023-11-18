/*
 *
 * Copyright (c) {11/8/23, 10:59 AM} Lorenzo A. Banks and Preston Thorpe. All rights reserved.
 * {IconButton.tsx}
 * {IconButton.tsx}
 *
 * This software is protected by copyright laws and international copyright treaties, as
 * well as other intellectual property laws and treaties. The software is licensed, not sold.
 *
 * However, you are not permitted to use the software as is, or distribute it without
 * obtaining a license from the authors. Unauthorized use of the software may result in
 * severe civil and criminal penalties, and will be prosecuted to the maximum extent
 * possible under law.
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 *  BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 *  ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 *  CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *  SOFTWARE.
 *
 * In no case shall the authors or copyright holders be liable for any claim, damages or
 * other liability arising from, out of or in connection with the softwareor the use or
 * other dealings in the software.
 * ********************************************************************************
 */

import {Component, JSXElement} from "solid-js";
import {CustomIcon, IconTree} from "solid-icons";

type IconEndType = {
    position:"End"
};

type IconStartType ={
    position:"Start"
};

type IconOnlyType = {
  position:"Only"
};

export type IconButtonType = IconEndType|IconStartType|IconOnlyType;

export interface IconButtonProps {
    icon:IconTree;
    children:JSXElement;
    size?:number;
    iconPosition:IconButtonType;
    color?:string;
    onClick: (e:MouseEvent) => void;
}

export const IconButton: Component<IconButtonProps> = (props) => {

    return (
        <button onClick={props.onClick}>
            {props.iconPosition.position === "Start" ? (<CustomIcon class={props.color}
                                                                     src={props.icon}
                                                                     size={props.size} />) : false }
            {props.iconPosition.position === "Only" ? (<CustomIcon class={props.color} src={props.icon} size={props.size}  />) : props.children}
            {props.iconPosition.position === "End" ? (<CustomIcon class={props.color}
                                                        src={props.icon}
                                                        size={props.size} />) : false}
        </button>
    );

}