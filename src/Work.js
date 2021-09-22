import styled from "styled-components"

const StyledWork = styled.div`
  width: 600px;
  height: 600px;
  margin: auto;
  background-color: ${(props) => props.theme.teal};
`

const Work = () => <StyledWork />

export default Work
