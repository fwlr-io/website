import { useLocalStorageState } from "hooks.js"

import { ThemeProvider } from "styled-components"
import { dark, light } from "Themes.js"
import { Row } from "Flex.js"
import LeftPanel from "LeftPanel.js"
import Content from "Content.js"
import ThemeToggle from "ThemeToggle.js"

const App = () => {
  const [themeState, setTheme] = useLocalStorageState("theme", "dark")
  const theme = themeState === "dark" ? dark : light
  return (
    <ThemeProvider theme={theme}>
      <Row id="main">
        <LeftPanel size={1}></LeftPanel>
        <Content size={5}></Content>
      </Row>
      <ThemeToggle themeState={themeState} setTheme={setTheme} />
    </ThemeProvider>
  )
}

export default App
