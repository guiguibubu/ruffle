package flash.text {
    import flash.display.InteractiveObject;
    import __ruffle__.stub_setter;
    
    public class TextField extends InteractiveObject {
        internal var _styleSheet:StyleSheet;
    
        public function TextField() {
            this.init();
        }

        private native function init();
        
        public native function get alwaysShowSelection():Boolean;
        public native function set alwaysShowSelection(value:Boolean):void;

        public native function get autoSize():String;
        public native function set autoSize(value:String):void;
        
        public native function get background():Boolean;
        public native function set background(value:Boolean):void;
        
        public native function get backgroundColor():uint;
        public native function set backgroundColor(value:uint):void;
        
        public native function get border():Boolean;
        public native function set border(value:Boolean):void;
        
        public native function get borderColor():uint;
        public native function set borderColor(value:uint):void;
        
        public native function get bottomScrollV():int;
        
        public native function get condenseWhite():Boolean
        public native function set condenseWhite(value:Boolean):void

        public native function get defaultTextFormat():TextFormat;
        public native function set defaultTextFormat(value:TextFormat):void;
        
        public native function get displayAsPassword():Boolean;
        public native function set displayAsPassword(value:Boolean):void;
        
        public native function get embedFonts():Boolean;
        public native function set embedFonts(value:Boolean):void;
        
        public native function get htmlText():String;
        public native function set htmlText(value:String):void;
        
        public native function get length():int;
        
        public native function get maxScrollH():int;
        
        public native function get maxScrollV():int;
        
        public native function get maxChars():int;
        public native function set maxChars(value:int):void;

        public native function get mouseWheelEnabled():Boolean
        public native function set mouseWheelEnabled(value:Boolean):void
        
        public native function get multiline():Boolean;
        public native function set multiline(value:Boolean):void;
        
        public native function get restrict():String;
        public native function set restrict(value:String):void;
        
        public native function get scrollH():int;
        public native function set scrollH(value:int):void;
        
        public native function get scrollV():int;
        public native function set scrollV(value:int):void;
        
        public native function get selectable():Boolean;
        public native function set selectable(value:Boolean):void;
        
        public function get styleSheet():StyleSheet {
            return this._styleSheet;
        }
        public function set styleSheet(value:StyleSheet):void {
            this._styleSheet = value;
            stub_setter("flash.text.TextField", "styleSheet");
        }
        
        public native function get text():String;
        public native function set text(value:String):void;
        
        public native function get textColor():uint;
        public native function set textColor(value:uint):void;
        
        public native function get textHeight():Number;
        
        public native function get textWidth():Number;
        
        public native function get type():String;
        public native function set type(value:String):void;
        
        public native function get wordWrap():Boolean;
        public native function set wordWrap(value:Boolean):void;
        
        public native function get antiAliasType():String;
        public native function set antiAliasType(value:String):void;
        
        public native function get gridFitType():String;
        public native function set gridFitType(value:String):void;
        
        public native function get thickness():Number;
        public native function set thickness(value:Number):void;
        
        public native function get sharpness():Number;
        public native function set sharpness(value:Number):void;
        
        public native function get numLines():int;

        public native function appendText(text:String):void;
        public native function getLineMetrics(lineIndex:int):TextLineMetrics;
        public native function getTextFormat(beginIndex:int = -1, endIndex:int = -1):TextFormat;
        public native function setTextFormat(format:TextFormat, beginIndex:int = -1, endIndex:int = -1):void;
        public native function replaceSelectedText(value:String):void;
        public native function replaceText(beginIndex:int, endIndex:int, newText:String):void;
        public native function setSelection(beginIndex:int, endIndex:int):void;
    }
}
