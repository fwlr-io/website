import styled from "styled-components"

const StyledJunk = styled.div`
  background-color: ${(props) => props.theme.green};
  width: 600px;
  height: 600px;
  margin: auto;
`

const Junk = () => <StyledJunk />

export default Junk
