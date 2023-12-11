<template>
  <v-app>
    <v-row>
      <v-col cols="2"></v-col>
      <v-col cols="10">
        <v-navigation-drawer :permanent="true">
          <v-list-item>Text Editor</v-list-item>
          <v-divider></v-divider>
          <v-list-item @click="openFileDialog">
            <v-icon>mdi-folder</v-icon> Open
          </v-list-item>
          <v-list-item @click="handleClickSave">
            <v-icon>mdi-content-save</v-icon> Save
          </v-list-item>
          <v-list-item @click="handleClickSaveAs">
            <v-icon>mdi-archive</v-icon> Save As
          </v-list-item>
          <v-list-item @click="dialog_create = true;">
            <v-icon>mdi-file</v-icon> New
          </v-list-item>
        </v-navigation-drawer>
        <v-container class="d-flex flex-column">
        <p class="text-subtitle-2">{{file_path}}</p>
        <v-divider class="ma-4"></v-divider>
          <v-textarea class="h-100" v-model="txt" clearable :on-click:clear="() => file_path=''"></v-textarea>
        </v-container>
      </v-col>
    </v-row>
    <v-dialog v-model="dialog_save" persistent transition="scale-transition" width="500">
      <v-card>
        <v-card-title>Are you sure that you want to save the file?</v-card-title>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn variant="outlined" @click="saveFile"><!--TODO: Open/Close the pop-up on the specific button-->
            Yes
          </v-btn>
          <v-btn variant="outlined" @click="handleNo">
            No
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-dialog v-model="dialog_save_as" persistent transition="scale-transition" width="500">
      <v-card>
        <v-card-title>Are you sure that you want to save the file?</v-card-title>
        <v-card-actions>
          <v-spacer></v-spacer>
          <input
          type = "text"
          v-model="new_file_path"
          placeholder="Enter name for file (leave empty for the same name)" />
        <v-btn @click="saveFileAs">Create</v-btn>
        <v-btn variant="outlined" @click="handleNo">
            No
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-dialog v-model="dialog_fail" persistent transition="scale-transition" width="auto">
      <v-card>
        <v-card-title>{{ message }}</v-card-title>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn height="50" width="500" @click="dialog_fail = false;">
            Ok
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-dialog v-model="dialog_create" persistent transition="scale-transition" width="auto">
    <v-card>
      <v-card-title>Enter name for new file:</v-card-title>
      <v-card-actions>
        <v-spacer></v-spacer>
        <input
          type = "text"
          v-model="file_path"
          placeholder="Enter name for new file" />
        <v-btn @click="createNewFile">Create</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
  </v-app>
</template>

<script lang="ts">
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { readTextFile } from "@tauri-apps/api/fs";

export default {
  data() {
    return {
      txt: "",
      file_path: "",
      new_file_path: "",
      dialog_save: false,
      dialog_fail: false,
      dialog_create: false,
      dialog_save_as: false,
      message: "",
    }
  },

  methods: {
    async openFileDialog() {
      try {
    const selectedPath = await open({
      multiple: false,
    });
    if (!selectedPath) return;
    this.txt = await readTextFile(selectedPath as string);
    // TODO: Set the path to the selcted file
    this.file_path = selectedPath as string;
    
  } catch (err) {
    console.error(err);
  }
    },

    async saveFile() {
      // TODO: Invoke the save_file function defined in tauri
      try{
        if(!this.file_path) {
          this.dialog_fail = true;
          this.message = "Please select a file to save";
          return;
        }
        const result = await invoke("save_file", {
          fileName: this.file_path,
          contents: this.txt,
        });
        this.dialog_save = false;
        console.log(result);
      } catch(error) {
        console.error("Error saving file: ", error);
        this.dialog_fail = true;
        this.file_path = "";
        this.message = "Error saving file!";
      }
    },

    async saveFileAs() {
      const result = await invoke("save_file_as", {
        fileName: this.file_path,
        newFileName: this.new_file_path,
      });
      console.log(result);
      this.dialog_save_as = false;
      return;
    },

    async createNewFile() {
      try {
        // TODO: Invoke the create_new_file function from Rust and set the file_path to waht this function returns 

        if (!this.file_path) return;
        let result = await invoke("create_new_file", {
          fileName: this.file_path,
          contents: this.txt,
        });
        this.txt = await readTextFile(this.file_path as string);
        
        this.dialog_create = false;
        console.log(result);
        // TODO: Close the dialog

      } catch (error) {
        this.dialog_create = false;
        console.log("Error is " + error);
        this.dialog_fail = true;
        this.message = "Error creating file!";
        this.file_path = "";
      }
    },

    handleNo() {
      this.dialog_save = false;
      this.dialog_save_as = false;
    },

    handleClickSave() {
      if(!this.file_path)
      {
        this.message = "Please open a file to save";
        this.dialog_fail = true;
        return;
      }
      this.dialog_save = true;
      return;
    },

    handleClickSaveAs() {
      if(!this.file_path)
      {
        this.message = "Please open a file to save";
        this.dialog_fail = true;
        return;
      }
      this.dialog_save_as = true;
      return;
    }

  }
}

</script>