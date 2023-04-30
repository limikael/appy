pub struct FlowAnchor {
    x: i32,
    y: i32,
    line_height: i32,
    pub elements: Vec<(i32, i32, i32, i32, Elements)>
}

impl FlowAnchor {
    pub fn new()->Self {
        Self {
            x: 0,
            y: 0,
            line_height: 0,
            elements: vec![]
        }
    }

    pub fn advance(&mut self, w:i32, h:i32, max_w:i32)->(i32,i32) {
        if self.x+w>max_w {
            self.x=0;
            self.y+=self.line_height;
            self.line_height=0;
        }

        let res=(self.x,self.y);

        self.x+=w;
        self.line_height=max(self.line_height,h);

        res
    }
}

#[derive(Clone)]
pub struct BlockContext {
    pub flow_anchor: Rc<RefCell<FlowAnchor>>,
    pub rect: Rect<i32>,
}

