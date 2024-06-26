<template>
    <div>
        <h2 class="text-blue-600">Wpisy Na Bloga</h2>
        <button @click="pobierzWpisy">Refresh</button>
        <div v-for="wpis in wpisy">
            <p>{{wpis}}</p>
        </div>
        {{ wpisy }}
        <input type="text" v-model="nowyBlog">
        <button @click="dodajWpis">Dodaj</button>
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