import { createGlobalStyle } from "styled-components"
import { darkBackground } from "ThemeDefs.js"

const GlobalStyle = createGlobalStyle`
  html,
  body,
  #root,
  #main {
    background-color: ${darkBackground};
  }
`

export default GlobalStyle
