import { useMemo } from "react"
import { useRoutes } from "react-router-dom"
import { useLocalStorageState } from "hooks.js"
import { dark, light } from "Themes.js"
import styled, { ThemeProvider } from "styled-components"
import { animated, useSpring, config } from "react-spring"
import ThemeToggle from "ThemeToggle.js"
import LeftPanel from "LeftPanel.js"
import Info from "Info.js"
import Work from "Work.js"
import Blog from "Blog.js"
import Junk from "Junk.js"

const Content = styled(animated.div)`
  flex: ${(props) => props.size};
  padding-top: 48px;
`
const contentAnim = (theme) => ({
  backgroundColor: theme.bgDark,
  color: theme.fgDefault,
})

const Row = styled.div`
  display: flex;
  flex-direction: row;
`

const App = () => {
  // setup dark mode / light mode
  const [themeState, setTheme] = useLocalStorageState("theme", "dark")
  const theme = themeState === "dark" ? dark : light

  const memoConfig = useMemo(
    () => ({
      config,
      ...contentAnim(theme),
    }),
    [theme]
  )
  const spring = useSpring(memoConfig)

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
        <Content style={spring} size={6}>
          {element}
        </Content>
      </Row>
      <ThemeToggle themeState={themeState} setTheme={setTheme} />
    </ThemeProvider>
  )
}

export default App
