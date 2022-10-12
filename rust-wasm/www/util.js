function jsFn() {
    return "JSFn";
  }
  
  class JsClass {
    constructor() {
    }
    getRandomNumber() {
      return Math.random()
    }
    getDate() {
      return Date.now()
    }
    getClassName() {
      return JsClass.name;
    }
  }
  
  export { jsFn, JsClass };
  