import Vue from 'vue'
import './plugins/axios'
import './plugins/vuetify'
import App from './App.vue'
import router from './router'
import './registerServiceWorker'

Vue.config.productionTip = false
var app;
console.log("start");
Vue.prototype.$tg = wasm_bindgen;
Vue.prototype.$http = axios;

const run = async function () {
  console.log("run");
  await Vue.prototype.$tg('textglue_wasm_bg.wasm');
  console.log("loaded");
  app = new Vue({
    router,
    render: h => h(App),
    data: {
      tg: null,
      loaded_wasm: true,
      data: {},
      status: "OK",
      status_color: "",
      message: ""      
    },
    methods: {
      error: function (message, reason) {
        if (message == "OK") {
          this.info(message);
        }
        else {
          this.status = "ERROR";
          this.status_color = "red";
          this.message = message;
          console.log("ERROR:" + message, reason);
        }
      },
      result: function (message) {
        if (message == "OK") {
          this.info(message);
        }
        else {
          this.error(message, "")
        }
      },
      info: function (message) {
        this.status = "OK";
        this.status_color = "green";
        this.message = message;
        console.log("INFO:" + message);
      },
      load: function () {
        this.info("Loading");
        this.$http.get("/api/db.json").then(
          function (response) {
            this.info("Response received");
            console.log(response.data);
            this.error(this.$tg.set_database(response.data));
            this.data = this.$tg.get_database();
          }.bind(this),
          function (err) {
            this.error("Loading error", err);
          }.bind(this)
        );
      },
      save: function () {
        this.$http.post("/api/upload-json", this.$tg.get_database()).then(
          function (response) {
            this.info("Save response received");
            console.log(response.data);
          }.bind(this),
          function (err) {
            this.error("Save error", err);
          }.bind(this)
        );
      }
    }
  }).$mount('#app');
};
run();

/*
app = new Vue({
  router,
  render: h => h(App)
}).$mount('#app');
*/