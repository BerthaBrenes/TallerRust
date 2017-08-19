var addon = require("../native");

module.exports = {
    fibonacci: function(int){
        return addon.fibonacci(int);
    }
}
console.log(addon);
