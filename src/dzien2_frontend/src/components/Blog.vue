<template>
    <div>
        <h2 class="text-blue-600">Wpisy Na Bloga</h2>
        <div class="w-100 flex flex-row-reverse">
            <button @click="pobierzWpisy" class="bg-blue-600 rounded-sm text-white p-4">Refresh</button>
        </div>
        <div class="grid mx-6 gap-4 my-4">
            <div v-for="(wpis, index) in wpisy" class="drop-shadow-xl bg-stone-300 p-4">
                <p>ID : {{index}}</p>
                <p>{{wpis}}</p>
            </div>
        </div>
        <div class="grid items-center justify-center gap-4 flex-col">
            <input type="text" v-model="nowyBlog" class="border-2 border-blue-600 p-4">
            <button class="bg-blue-600 rounded-sm text-white p-4" @click="dodajWpis">Dodaj</button>
        </div>
    </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';

    export default {
        data() {
            return {
                wpisy: [],
                nowyBlog: ""
            }
        },
        methods: {
            async dodajWpis() {
                await dzien2_backend.dodaj_wpis(this.nowyBlog);
            },
            async pobierzWpisy() {
                this.wpisy = await dzien2_backend.odczytaj_wpisy();
            }
        },
        async mounted(){
            this.pobierzWpisy();
        }
    }
</script>