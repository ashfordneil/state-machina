// Project Dependancies
import './main.css'
import { Main } from "./Main"
import setupVisPorts from "./Native/Vis"

// Launch project code
const main = Main.fullscreen()

// ports
setupVisPorts(main.ports)