<template>
    <v-card>
        <v-card-title><h2>{{chapter.name}}</h2></v-card-title>
  <v-layout row wrap>
    <v-flex xs12>
      <v-card v-for="(item, index) in chapter.snippets">
        <v-card-title primary-title>
            {{metadata[item].name}} <em>({{item}})</em>
        </v-card-title>
        <v-card-text>
            {{metadata[item].summary}}
            <v-btn fab small @click="remove(index)"><v-icon>remove_circle</v-icon></v-btn>
            <v-btn fab small @click="up(index)"><v-icon>arrow_upward</v-icon></v-btn>
            <v-btn fab small @click="down(index)"><v-icon>arrow_downward</v-icon></v-btn>
        </v-card-text>
      </v-card>
    </v-flex>
  </v-layout>            <!--
            <v-list two-line>
                <v-list-tile>
                    <v-list-tile-title><h2>{{chapter.name}}</h2></v-list-tile-title>
                </v-list-tile>
                <template v-for="(item, index) in chapter.snippets">
                    <v-divider :key="index"></v-divider>
                    <v-list-tile :key="index">
                        <v-list-tile-content>
                            <v-list-tile-title>{{metadata[item].name}} <em>({{item}})</em></v-list-tile-title>
                            <v-list-tile-sub-title>{{metadata[item].summary}}</v-list-tile-sub-title>
                            <v-btn fab small @click="remove(index)"><v-icon>remove_circle</v-icon></v-btn>
                            <v-btn fab small @click="up(index)"><v-icon>arrow_upward</v-icon></v-btn>
                            <v-btn fab small @click="down(index)"><v-icon>arrow_downward</v-icon></v-btn>
                        </v-list-tile-content>
                    </v-list-tile>
                </template>
                <v-divider></v-divider>
            </v-list>
            -->        
        </v-card-title>
        <SnippetSelector v-model="add_snippet" :snippet_ids="snippet_ids"></SnippetSelector>
        <v-card-actions>
            <v-btn @click="add()">Add</v-btn>
        </v-card-actions>
    </v-card>
</template>

<script>
  import SnippetSelector from "./SnippetSelector";

  export default {
    components: {
        SnippetSelector
    },
    data: () => ({
        chapter:{},
        metadata:{},
        add_snippet:"",
        snippet_ids:[]
    }),
    props:["document","chapter_number"],

    created() {
        console.log("DOC",this.document,this.$tg.get_document(this.document));
        this.update();
    },
    methods: {
        add(){
            this.chapter = this.$tg.get_chapter(this.document,this.chapter_number);
            this.chapter.snippets.push(this.add_snippet);
            this.$tg.set_chapter(this.document,this.chapter_number,this.chapter);
            this.update();
        },
        remove(index){
//            this.chapter.snippets = this.chapter.snippets.filter(x => x!=id);
            this.chapter.snippets.splice(index,1);
            this.$tg.set_chapter(this.document,this.chapter_number,this.chapter);
            this.update();
        },
        up(index){
            console.log("up",index);
            if (index>=1){
                var id = this.chapter.snippets[index-1];
                this.chapter.snippets[index-1]=this.chapter.snippets[index]
                this.chapter.snippets[index]=id;
                this.$tg.set_chapter(this.document,this.chapter_number,this.chapter);
                this.update();
            }
        },
        down(index){
            console.log("down",index);
            if (index<this.chapter.snippets.length-1){
                var id = this.chapter.snippets[index+1];
                this.chapter.snippets[index+1]=this.chapter.snippets[index]
                this.chapter.snippets[index]=id;
                this.$tg.set_chapter(this.document,this.chapter_number,this.chapter);
                this.update();
            }
        },
        update(){
            if (this.chapter_number<this.$tg.get_document(this.document).chapters.length){
                this.chapter = this.$tg.get_chapter(this.document,this.chapter_number);
            }
            else{
                this.chapter = {};
            }
            this.metadata=this.$tg.get_metadata();
            this.snippet_ids = Object.keys(this.metadata);
            this.snippet_ids.sort();
            this.snippet_ids.reverse();

        }
    },
    watch: {
        document(){
            this.update();
        },
        chapter_number(){
            this.update();
        }
    },
  }
</script>

<style>
</style>
