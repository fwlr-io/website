import styled, { ThemeProvider } from "styled-components"
import { useRoutes } from "react-router-dom"
import { useLocalStorageState } from "hooks.js"
import { dark, light } from "Themes.js"
import ThemeToggle from "ThemeToggle.js"

import { Row, Item } from "Flex.js"
import LeftPanel from "LeftPanel.js"
import Info from "Info.js"
import Work from "Work.js"
import Blog from "Blog.js"
import Junk from "Junk.js"

const Content = styled(Item)`
  background-color: ${(props) => props.theme.bgDark};
  color: ${(props) => props.theme.fgDefault};
  padding-top: 48px;
`

const App = () => {
  // setup dark mode / light mode
  const [themeState, setTheme] = useLocalStorageState("theme", "dark")
  const theme = themeState === "dark" ? dark : light

  // setup routing
  const element = useRoutes([
    { path: "info", element: <Info /> },
    { path: "work", element: <Work /> },
    { path: "blog", element: <Blog /> },
    { path: "junk", element: <Junk /> },
  ])

  return (
    <ThemeProvider theme={theme}>
      <Row id="main">
        <LeftPanel size={1} />
        <Content size={5}>{element}</Content>
      </Row>
      <ThemeToggle themeState={themeState} setTheme={setTheme} />
    </ThemeProvider>
  )
}

export default App
