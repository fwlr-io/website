import styled from "styled-components"
import { animated } from "react-spring"
import { useAnimated } from "hooks.js"

import Post from "blog/Post.js"
// import PostTwo from "blog/PostTwo.js"
// import PostThree from "blog/PostThree.js"

const StyledBlog = styled(animated.div)`
  line-height: 1.3;
  padding: 0 2em;
  display: flex;
  flex-flow: column;
  justify-content: start;
  gap: 2em;
`
const animBlog = (theme) => ({
  color: theme.lightForeground,
})

const BlogPost = () => {
  const blogPostAnim = useAnimated(animBlog)
  return (
    <StyledBlog style={blogPostAnim}>
      <Post />
      {/* <PostTwo /> */}
      {/* <PostThree /> */}
    </StyledBlog>
  )
}

export default BlogPost
