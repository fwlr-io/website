import { Children } from "react"
import styled from "styled-components"
import { blue } from "ThemeDefs.js"

const StyledParagraph = styled.div`
  display: flex;
  flex-flow: row wrap;
  justify-content: space-around;
`
const ParagraphContainer = styled.div`
  width: 300px;
  flex: none;
  display: flex;
  justify-content: start;
`

export const MobParagraph = ({ children }) => {
  return (
    <StyledParagraph>
      {Children.map(children || null, (child, i) => (
        <ParagraphContainer>{child}</ParagraphContainer>
      ))}
    </StyledParagraph>
  )
}

export const MobSerif = styled.p.attrs({ className: "serif" })``

export const MobCode = styled.code``

export const MobImage = styled.div`
  width: 200px;
  height: 200px;
  background-color: ${blue};
`
