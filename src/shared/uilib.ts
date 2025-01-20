import Button from "primevue/button"
import { Plugin } from "vue";
import Card from 'primevue/card';
import Dialog from 'primevue/dialog';
import Tree from 'primevue/tree';
import InputText from 'primevue/inputtext';
import ContextMenu from 'primevue/contextmenu';
import Select from 'primevue/select';
import Menu from 'primevue/menu';
import SelectButton from 'primevue/selectbutton';
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';


const components = {
    Button,
    Card,
    Dialog,
    Tree,
    ContextMenu,
    Menu,
    Select,
    SelectButton,
    Splitter,
    SplitterPanel,
    InputText
};

export const setupUILib: Plugin = {
    install(app, options) {
        Object.entries(components).forEach(([key, val]) => {
            console.log(key, val);
            app.component(key, val);
        })
    }
}

