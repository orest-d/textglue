<template>
  <v-app dark>
    <v-toolbar dense>
      <img src="logo.png" ></img>
      <v-menu offset-y>
        <template v-slot:activator="{ on }">
          <v-toolbar-title v-on="on">
            <span>TextGlue</span>
            <v-icon>arrow_drop_down</v-icon>
          </v-toolbar-title>
        </template>
        <v-list>
          <v-list-tile @click="mode='snippets'">
            <v-list-tile-title>Snippets</v-list-tile-title>
          </v-list-tile>
          <v-list-tile @click="mode='documents'">
            <v-list-tile-title>Documents</v-list-tile-title>
          </v-list-tile>
        </v-list>
      </v-menu>

      <v-select v-model="selected_snippet" :items="snippet_ids" solo v-if="mode=='snippets'">
        <template slot="selection" slot-scope="data">
          <b>{{metadata[data.item].name}}</b>&nbsp;&nbsp;<em>({{data.item}})</em> 
        </template>
        <template slot="item" slot-scope="data">
          <b>{{metadata[data.item].name}}</b>&nbsp;&nbsp;<em>({{data.item}})</em> 
        </template>
      </v-select>

      <v-select v-model="selected_document" :items="document_names" solo v-if="mode=='documents'">
      </v-select>

      <v-btn @click="new_snippet()" v-if="mode=='snippets'"><v-icon>add</v-icon></v-btn>
      <v-btn @click="new_document()" v-if="mode=='documents'"><v-icon>add_box</v-icon></v-btn>

      <v-btn @click="save()"><v-icon>save_alt</v-icon></v-btn>
      <v-spacer></v-spacer>
      <v-switch v-model="ext" label="Extended"></v-switch>
      <v-tooltip bottom>
        <template v-slot:activator="{ on }">
          <v-chip :color="status_color" v-on="  on">{{status}}</v-chip>
        </template>
        <span>{{message}}</span>
      </v-tooltip>
    </v-toolbar>

    <v-content v-if="loaded_wasm">
      <v-container v-if="ext">
        <v-text-field v-model="snippet_name" label="Name"></v-text-field>
        <v-textarea v-model="snippet_summary" label="Summary"></v-textarea>
        <v-combobox v-model="snippet_tags" label="Tags" multiple chips></v-combobox>
      </v-container>
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
      mode:"snippets",
      loaded_wasm: true,
      data: { snippets: {} },
      status: "OK",
      status_color: "",
      message: "",
      metadata: {},
      snippet_ids: [],
      snippet_text: "",
      selected_snippet: "",
      ext:false
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
      this.status_color = "";
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
    },
    snippet_name: {
      get() {
        return this.$tg.get_metadata()[this.selected_snippet].name;
      },
      set(value) {
        var metadata=this.$tg.get_metadata()[this.selected_snippet];
        metadata.name = value;
        this.metadata[this.selected_snippet]=metadata;
        this.$tg.set_metadata(this.selected_snippet, metadata);
      }
    },
    snippet_summary: {
      get() {
        return this.$tg.get_metadata()[this.selected_snippet].summary;
      },
      set(value) {
        var metadata=this.$tg.get_metadata()[this.selected_snippet];
        metadata.summary = value;
        this.metadata[this.selected_snippet]=metadata;
        this.$tg.set_metadata(this.selected_snippet, metadata);
      }
    },
    snippet_tags: {
      get() {
        return this.$tg.get_metadata()[this.selected_snippet].tags;
      },
      set(value) {
        var metadata=this.$tg.get_metadata()[this.selected_snippet];
        metadata.tags = value;
        this.metadata[this.selected_snippet]=metadata;
        this.$tg.set_metadata(this.selected_snippet, metadata);
      }
    },
    document_names: {
      get() {
        return Object.keys(this.data.documents);
      }
    }
  },
  created() {
    this.$tg.set_snippet("abc", "lorem");
    this.load();
  }
};
</script>
