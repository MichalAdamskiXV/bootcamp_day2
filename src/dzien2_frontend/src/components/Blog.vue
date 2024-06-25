<template>
    <div>
        <button @click="pobierzWpisy">Click Me!</button>
        siema blog
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
    }
</script>