<template>
  <v-app dark>
    <v-toolbar dense app>
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
      <v-select v-model="selected_chapter" :items="chapters_id" item-value="id" solo v-if="mode=='documents'">
        <template slot="selection" slot-scope="data">
          <b>{{data.item.name}}</b> 
        </template>
        <template slot="item" slot-scope="data">
          <b>{{data.item.name}}</b> 
        </template>
      </v-select>
      <v-btn @click="new_snippet()" v-if="mode=='snippets'"><v-icon>add</v-icon></v-btn>
      <v-btn @click="new_chapter()" v-if="mode=='documents'"><v-icon>add_box</v-icon></v-btn>
      <v-btn @click="update()"><v-icon>update</v-icon></v-btn>
      <v-btn @click="text_to_chapter()">Text to chapter</v-btn>

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
<!--
      <SnippetSelector v-model='test' :snippet_ids="snippet_ids"></SnippetSelector>
      <h1>{{test}}</h1>
-->
    <v-content v-if="mode=='snippets'">

      <v-container v-if="ext">
        <v-text-field v-model="snippet_name" label="Name"></v-text-field>
        <v-textarea v-model="snippet_summary" label="Summary"></v-textarea>
        <v-combobox v-model="snippet_tags" label="Tags" multiple chips></v-combobox>
      </v-container>
      <v-textarea v-model="snippet_text" rows="45"></v-textarea>
    </v-content>
      <v-navigation-drawer v-if="mode=='documents'" app width="320">
      <ChapterEditor :document="selected_document" :chapter_number="selected_chapter"></ChapterEditor>
      </v-navigation-drawer>
    <v-content v-if="mode=='documents'">
      <v-textarea v-model="chapter_text" rows="45" label="Chapter text"></v-textarea>
    </v-content>
  </v-app>
</template>

<script>
import SnippetSelector from "./components/SnippetSelector";
import ChapterEditor from "./components/ChapterEditor";

export default {
  name: "App",
  components: {
    SnippetSelector, ChapterEditor
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
      document_names:[],
      chapters:[],
      chapters_id:[],
      selected_snippet: "",
      selected_document: "document",
      selected_chapter: 0,
      chapter_text:"Chapter text - initial",
      test:null,
      ext:false,
      id_prefix:"-----=====##### ",
      id_postfix:" #####=====-----\n",
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
    update(){
      //this.data = this.$tg.get_database();
      this.metadata = this.$tg.get_metadata();
      this.documents= this.$tg.get_documents();
      this.snippet_ids = Object.keys(this.metadata);
      this.snippet_ids.sort();
      this.snippet_ids.reverse();
      this.document_names = Object.keys(this.documents);
      this.chapters = this.$tg.get_document(this.selected_document).chapters;
      this.chapters_id =[];
      for (var i = 0; i<this.chapters.length; i++){
        this.chapters_id.push({...this.chapters[i],id:i});
      }
      this.selected_chapter_structure=this.$tg.get_chapter(this.selected_document,this.selected_chapter);
      this.$forceUpdate();
      this.chapter_text = this.$tg.get_chapter_text(this.selected_document,this.selected_chapter,this.id_prefix,this.id_postfix);
    },
    text_to_chapter(){
      this.$tg.set_chapter_text(this.selected_document,this.selected_chapter,this.id_prefix,this.id_postfix, this.chapter_text);
    },
    load(){
      if ("textglue_db" in localStorage){
        this.error(this.$tg.set_database_json(localStorage.textglue_db));
        if (this.status!="OK"){
          this.load_from_server();
        }
        else{
          this.update();
        }
      }
      else{
        this.load_from_server();
      }
    },
    load_from_server() {
      this.info("Loading");
      this.$http.get("/api/db.json").then(
        function(response) {
          this.info("Response received");
          console.log(response.data);
          this.error(this.$tg.set_database(response.data));
          this.update();
        }.bind(this),
        function(err) {
          this.error("Loading error", err);
        }.bind(this)
      );
    },
    save() {
      let db = this.$tg.get_database();
      let db_json = this.$tg.get_database_json();
      localStorage.setItem("textglue_db",db_json);
      this.$http.post("/api/upload-json", db).then(
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
    },
    new_snippet(){
      let id = (new Date()).toISOString().slice(0,10);
      this.selected_snippet=this.$tg.new_snippet(id);
      this.update();
      this.info("New snippet: "+id);
    },
    new_chapter(){
      var ch = this.$tg.add_chapter_autoname(this.selected_document);
      this.selected_chapter=this.$tg.get_document(this.selected_document).chapters.length-1;
      this.update();
      this.info("New chapter "+ch.name);
    }
  },
  computed: {
    snippet_text: {
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
    chapter_snippets:{
      get(){
        this.chapters[this.selected_chapter].snippets;
      },
      set(value){
        this.result(this.$tg.set_chapter(this.selected_document,this.selected_chapter,value));
        if (this.status=="OK"){
          this.chapters[this.selected_chapter].snippets=value;
        }
      },

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
    }
  },
  created() {
    this.load();
  }
};
</script>
