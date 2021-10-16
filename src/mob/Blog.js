import styled from "styled-components"
import { animated } from "react-spring"
import { useAnimated } from "hooks.js"

import Post from "blog/Post.js"
// import PostTwo from "blog/PostTwo.js"
// import PostThree from "blog/PostThree.js"

const StyledBlog = styled(animated.div)`
  padding: 0 1em;
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
