import styled from "styled-components"
import { animated } from "react-spring"
import { useAnimated } from "hooks.js"
import { Outlet } from "react-router-dom"
import LeftPanel from "./LeftPanel.js"
import ModeToggle from "./ModeToggle.js"

const Content = styled(animated.div)`
  flex: auto;
  padding-top: 48px;
  width: 100%;
`
const animContent = (theme) => ({
  backgroundColor: theme.mediumBackground,
  color: theme.mediumForeground,
})

const Row = styled.div`
  display: flex;
  flex-direction: row;
`

const Layout = () => {
  const anim = useAnimated(animContent)
  return (
    <>
      <Row id="main">
        <LeftPanel />
        <Content style={anim}>
          <Outlet />
        </Content>
      </Row>
      <ModeToggle />
    </>
  )
}

export default Layout
