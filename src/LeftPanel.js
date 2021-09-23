import styled from "styled-components"
import { animated } from "react-spring"
import { useThemeSpring } from "hooks.js"
import { NavLink } from "react-router-dom"

const LeftPanelDiv = styled(animated.div)`
  display: flex;
  flex-direction: column;
  flex: ${(props) => props.size};
  align-items: stretch;
  gap: 2em;
  min-width: 180px;
`
const leftPanelDivAnim = (theme) => ({
  backgroundColor: theme.bgDefault,
  borderRight: `1px solid ${theme.fgDefault}`,
})

const MenuItem = styled(NavLink)`
  font-size: 3em;
  text-align: center;
  text-decoration: none;
  color: ${(props) => props.theme.fgDark};
  &:hover {
    color: ${(props) => props.theme.teal};
  }
  &.active {
    color: ${(props) => props.theme.green};
  }
`

const TopMenuItem = styled(MenuItem)`
  font-size: 4em;
  font-weight: bold;
  padding: 12px 0px;
  border-bottom: 1px solid ${(props) => props.theme.fgDefault};
`

const LeftPanel = ({ size }) => {
  const spring = useThemeSpring(leftPanelDivAnim)
  return (
    <LeftPanelDiv style={spring} size={size} className="code">
      <TopMenuItem to="fwlr">fwlr</TopMenuItem>
      <MenuItem to="info">info</MenuItem>
      <MenuItem to="work">work</MenuItem>
      <MenuItem to="blog">blog</MenuItem>
      <MenuItem to="junk">junk</MenuItem>
    </LeftPanelDiv>
  )
}
export default LeftPanel
