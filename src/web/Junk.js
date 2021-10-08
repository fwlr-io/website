import styled from "styled-components"
import { green } from "ThemeDefs.js"

const StyledJunk = styled.div`
  background-color: ${green};
  width: 600px;
  height: 600px;
  margin: auto;
`

const Junk = () => <StyledJunk />

export default Junk
