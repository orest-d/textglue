<template>
  <v-app dark>
    <v-toolbar>
      <v-toolbar-title>TextGlue</v-toolbar-title>
      <v-btn @click="save()">Save</v-btn>
      <v-select v-model="selected_snippet" :items="snippet_ids" solo>
        <template slot="selection" slot-scope="data">
          <b>{{metadata[data.item].name}}</b>&nbsp;&nbsp;<em>({{data.item}})</em> 
        </template>
        <template slot="item" slot-scope="data">
          <b>{{metadata[data.item].name}}</b>&nbsp;&nbsp;<em>({{data.item}})</em> 
        </template>
      </v-select>
      <v-spacer></v-spacer>
      <v-tooltip bottom>
        <template v-slot:activator="{ on }">
          <v-chip :color="status_color" v-on="  on">{{status}}</v-chip>
        </template>
        <span>{{message}}</span>
      </v-tooltip>
    </v-toolbar>

    <v-content v-if="loaded_wasm">
      <v-textarea v-model="value" rows="45"></v-textarea>
    </v-content>
    <v-content v-else>
      <v-container bg fill-height grid-list-md text-xs-center>
        <v-layout row wrap align-center>
          <v-card style="width:80%;height:80%;">
            <h1>Loading</h1>
          </v-card>
        </v-layout>
      </v-container>
      <HelloWorld/>
    </v-content>
  </v-app>
</template>

<script>
import HelloWorld from "./components/HelloWorld";

export default {
  name: "App",
  components: {
    HelloWorld
  },
  data() {
    return {
      //
      loaded_wasm: true,
      data: { snippets: {} },
      status: "OK",
      status_color: "",
      message: "",
      metadata: {},
      snippet_ids: [],
      snippet_text: "",
      selected_snippet: ""
    };
  },
  methods: {
    error(message, reason) {
      if (message == "OK") {
        this.info(message);
      } else {
        this.status = "ERROR";
        this.status_color = "red";
        this.message = message;
        console.log("ERROR:" + message, reason);
      }
    },
    result(message) {
      if (message == "OK") {
        this.info(message);
      } else {
        this.error(message, "");
      }
    },
    info(message) {
      this.status = "OK";
      this.status_color = "green";
      this.message = message;
      console.log("INFO:" + message);
    },
    load() {
      this.info("Loading");
      this.$http.get("/api/db.json").then(
        function(response) {
          this.info("Response received");
          console.log(response.data);
          this.error(this.$tg.set_database(response.data));
          this.data = this.$tg.get_database();
          this.metadata = this.$tg.get_metadata();
          this.snippet_ids = Object.keys(this.metadata);
        }.bind(this),
        function(err) {
          this.error("Loading error", err);
        }.bind(this)
      );
    },
    save() {
      this.$http.post("/api/upload-json", this.$tg.get_database()).then(
        function(response) {
          this.info("Save response received");
          console.log(response.data);
        }.bind(this),
        function(err) {
          this.error("Save error", err);
        }.bind(this)
      );
    },
    snippet_item(snippet_id) {
      console.log("snippet item", snippet_id);
      return snippet_id + ": " + this.metadata[snippet_id].name;
    }
  },
  computed: {
    value: {
      get() {
        return this.$tg.get_snippet(this.selected_snippet);
      },
      set(value) {
        this.$tg.set_snippet(this.selected_snippet, value);
      }
    }
  },
  created() {
    this.$tg.set_snippet("abc", "lorem");
    this.load();
  }
};
</script>
