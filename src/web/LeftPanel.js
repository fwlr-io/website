import styled from "styled-components"
import { animated } from "react-spring"
import { useAnimated } from "hooks.js"
import { cyan, green } from "ThemeDefs.js"
import { NavLink } from "react-router-dom"

const MenuItem = styled(NavLink)`
  font-size: 3rem;
  text-align: center;
  text-decoration: none;
  color: inherit;
  &:hover {
    color: ${cyan};
  }
  &.active {
    color: ${green};
  }
`

const TopMenuItem = styled(MenuItem)`
  font-size: 4rem;
  font-weight: bold;
  padding-top: 1rem;
`

const LeftPanelDiv = styled(animated.div)`
  flex: none;
  width: 20ch;
  display: flex;
  flex-direction: column;
  align-items: stretch;
  gap: 2rem;
`
const animLeftPanelDiv = (theme) => ({
  backgroundColor: theme.darkBackground,
  color: theme.darkForeground,
})

const LeftPanel = () => {
  const anim = useAnimated(animLeftPanelDiv)
  return (
    <LeftPanelDiv style={anim} className="code">
      <TopMenuItem to="/" end>
        fwlr
      </TopMenuItem>
      <MenuItem to="info">info</MenuItem>
      <MenuItem to="work">work</MenuItem>
      <MenuItem to="blog">blog</MenuItem>
      <MenuItem to="junk">junk</MenuItem>
    </LeftPanelDiv>
  )
}

export default LeftPanel
