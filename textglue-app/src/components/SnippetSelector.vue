<template>
  <v-select
      v-bind:snippet_id="snippet_id"
      v-on:input="$emit('input',$event)"
      :items="snippet_ids" solo>
    <template slot="selection" slot-scope="data">

      <b v-if="data.item in metadata">{{metadata[data.item].name}}</b>&nbsp;&nbsp;
      <em>({{data.item}})</em>
    </template>
    <template slot="item" slot-scope="data">
      <b v-if="data.item in metadata">{{metadata[data.item].name}}</b>&nbsp;&nbsp;
      <em>({{data.item}})</em>
    </template>
  </v-select>
</template>

<script>
  export default {
    data: () => ({
        metadata:{},        
    }),
    props:["snippet_id","snippet_ids"],
    methods:{
        update(){
            //this.data = this.$tg.get_database();
            this.metadata = this.$tg.get_metadata();
            this.snippet_ids = Object.keys(this.metadata);
            this.snippet_ids.sort();
            this.snippet_ids.reverse();
        }
    },
    created() {
      this.update();
    },
  }
</script>

<style>
</style>
