import styled from "styled-components"
import { animated } from "react-spring"
import { useAnimated } from "hooks.js"
import { Outlet } from "react-router-dom"

import NavBar from "./NavBar.js"

const Main = styled.div`
  display: flex;
  flex-direction: column;
  align-items: stretch;
`

const Content = styled(animated.div)`
  flex: auto;
  min-height: 100vh;
  padding: 1rem 1ch;
`
const animContent = (theme) => ({
  backgroundColor: theme.mediumBackground,
  color: theme.mediumForeground,
})

const Layout = () => {
  const anim = useAnimated(animContent)
  return (
    <Main>
      <NavBar />
      <Content style={anim}>
        <Outlet />
      </Content>
    </Main>
  )
}

export default Layout
